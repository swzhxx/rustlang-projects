use std::{collections::HashMap, pin::Pin};

use amf::amf0;
use anyhow::Result;
use dashmap::DashMap;
use once_cell::sync::OnceCell;
use tokio::{
    io::{AsyncRead, AsyncReadExt},
    net::TcpStream,
    stream,
};

use crate::{
    rtmp::protocol::handshark::{HandShark1, HandShark2},
    util::{gen_random_bytes, AsyncFrom, AsyncWriteByte, EventBus, AR, AW},
};

use self::{
    chunk::{Chunk, FullChunkMessageHeader},
    handshark::{HandShark0, HandSharkState},
    message::{Message, MessageFactor},
    read_effect::AsyncReaderEffect,
};
mod chunk;

pub mod handshark;
pub mod message;

mod read_effect;

pub fn eventbus_map() -> &'static DashMap<String, EventBus<Message>> {
    static INSTANCE: OnceCell<DashMap<String, EventBus<Message>>> = OnceCell::new();
    INSTANCE.get_or_init(|| DashMap::new())
}

pub fn video_header_map() -> &'static DashMap<String, Message> {
    static INSTANCE: OnceCell<DashMap<String, Message>> = OnceCell::new();
    INSTANCE.get_or_init(|| DashMap::new())
}

pub fn audio_header_map() -> &'static DashMap<String, Message> {
    static INSTANCE: OnceCell<DashMap<String, Message>> = OnceCell::new();
    INSTANCE.get_or_init(|| DashMap::new())
}

pub fn meta_data_map() -> &'static DashMap<String, RtmpMetaData> {
    static INSTANCE: OnceCell<DashMap<String, RtmpMetaData>> = OnceCell::new();
    INSTANCE.get_or_init(|| DashMap::new())
}

#[derive(Debug)]
pub struct RtmpCtx {
    ctx_begin_timestamp: i64,
    pub last_full_chunk_message_header: HashMap<u32, chunk::FullChunkMessageHeader>,
    chunk_size: u32,
    pub reve_bytes: usize,
    abort_chunk_id: Option<u32>,
    stream_name: Option<String>,
    is_publisher: bool,
    is_delete: bool,
}

#[derive(Debug, Default)]
pub struct RtmpMetaData {
    pub width: f64,
    pub height: f64,
    pub video_codec_id: String,
    pub video_data_rate: f64,
    pub audio_codec_id: String,
    pub audio_data_rate: f64,
    pub audio_sample_rate: f64,
    pub audio_sample_size: f64,
    pub stereo: bool,
    pub frame_rate: f64,
    pub duration: f64,

    pub file_size: f64,
    pub major_brand: String,
    pub minor_version: String,
    pub compatible_brands: String,
    pub encoder: String,
}

impl TryFrom<&amf0::Value> for RtmpMetaData {
    type Error = anyhow::Error;

    fn try_from(value: &amf0::Value) -> Result<Self, Self::Error> {
        let mut meta_data = RtmpMetaData::default();
        if let amf0::Value::EcmaArray { entries } = value {
            for item in entries {
                match item.key.as_ref() {
                    "duration" => {
                        meta_data.duration = item.value.try_as_f64().unwrap_or_default();
                    }
                    "width" => {
                        meta_data.width = item.value.try_as_f64().unwrap_or_default();
                    }
                    "height" => {
                        meta_data.height = item.value.try_as_f64().unwrap_or_default();
                    }
                    "videocodecid" => {
                        meta_data.video_codec_id =
                            item.value.try_as_str().unwrap_or_default().to_owned();
                    }
                    "videorate" => {
                        meta_data.video_data_rate = item.value.try_as_f64().unwrap_or_default();
                    }
                    "framerate" => {
                        meta_data.video_data_rate = item.value.try_as_f64().unwrap_or_default();
                    }
                    "audiocodeid" => {
                        meta_data.audio_codec_id =
                            item.value.try_as_str().unwrap_or_default().to_owned();
                    }
                    "audiodatarate" => {
                        meta_data.audio_data_rate = item.value.try_as_f64().unwrap_or_default();
                    }
                    "audio_sample_rate" => {
                        meta_data.audio_sample_rate = item.value.try_as_f64().unwrap_or_default();
                    }
                    "audio_sample_size" => {
                        meta_data.audio_sample_size = item.value.try_as_f64().unwrap_or_default();
                    }
                    "encoder" => {
                        meta_data.encoder = item.value.try_as_str().unwrap_or_default().to_owned();
                    }
                    "compatible_brands" => {
                        meta_data.compatible_brands =
                            item.value.try_as_str().unwrap_or_default().to_owned();
                    }
                    "stereo" => match item.value {
                        amf::Amf0Value::Boolean(b) => {
                            meta_data.stereo = b;
                        }
                        _ => {}
                    },
                    "minor_version" => {
                        meta_data.minor_version =
                            item.value.try_as_str().unwrap_or_default().to_owned();
                    }
                    _ => {}
                }
            }
        }
        Ok(meta_data)
    }
}

impl RtmpCtx {
    fn new() -> Self {
        Self {
            ctx_begin_timestamp: chrono::Local::now().timestamp_millis(),
            last_full_chunk_message_header: HashMap::default(),
            chunk_size: 128,
            reve_bytes: 0,
            abort_chunk_id: None,
            stream_name: None,
            is_publisher: false,
            is_delete: false,
        }
    }
}

impl RtmpCtx {
    async fn handle_hand_check(stream: &mut TcpStream) -> anyhow::Result<()> {
        let mut receive_handshark_state: Option<HandSharkState> = None;
        let mut send_handshark_state: Option<HandShark1> = None;
        let begin_timestamp = chrono::Local::now().timestamp_millis();
        loop {
            match receive_handshark_state {
                Some(rhs) => match rhs {
                    HandSharkState::C0(_) => {
                        let s0 = HandShark0::default();
                        s0.async_write_byte(stream).await;
                        log::trace!("[SEND]->S0");
                        let mut s1 = HandShark1::default();
                        s1.time =
                            (chrono::Local::now().timestamp_millis() - begin_timestamp) as u32;
                        s1.random_data = gen_random_bytes(1528);
                        s1.async_write_byte(stream).await;
                        log::trace!("[SEND] -> S1");
                        send_handshark_state = Some(s1);

                        let c1 = HandShark1::async_from(stream).await;
                        log::trace!("[RECEIVE] -> C1 time:{:?}", c1.time);
                        receive_handshark_state = Some(HandSharkState::C1(c1));
                    }
                    HandSharkState::C1(c1) => {
                        let s2 = HandShark2 {
                            time1: c1.time,
                            time2: 0,
                            random_echo: c1.random_data.clone(),
                        };

                        s2.async_write_byte(stream).await;
                        log::trace!("[SEND] -> S2 ");
                        let c2 = HandShark2::async_from(stream).await;
                        log::trace!("[RECEIVE] -> C2 time1:{:?} time2{:?}", c2.time1, c2.time2);
                        receive_handshark_state = Some(HandSharkState::C2(c2));
                    }
                    HandSharkState::C2(c2) => {
                        // 比对数据
                        assert_eq!(
                            c2.random_echo,
                            send_handshark_state.as_ref().unwrap().random_data
                        );
                        log::info!("[HANDSHARK SUCCESS]");
                        break;
                    }
                    _ => {
                        unreachable!()
                    }
                },
                None => {
                    let c0 = HandShark0::async_from(stream).await;
                    log::trace!("[RECEIVE]-> C0 version:{:?}", c0.version);
                    receive_handshark_state = Some(HandSharkState::C0(c0));
                }
            }
        }
        Ok(())
    }
}

impl RtmpCtx {
    async fn handle_receive_chunk(&mut self, stream: &mut TcpStream) -> anyhow::Result<()> {
        let mut message_factor = MessageFactor::new();
        loop {
            let (chunk, full_chunk_message_header) = {
                let mut effect_reader = AsyncReaderEffect::new(stream);
                let result = Chunk::async_read_chunk(&mut effect_reader, self).await?;
                self.reve_bytes += effect_reader.get_readed_bytes_num();
                log::info!(
                    "[RECEIVED MESSAGE ] -> TYPE {:?} ; CHUNK ID {:?}",
                    result.1.message_type,
                    result.0.cs_id
                );

                result
            };
            self.last_full_chunk_message_header
                .insert(chunk.cs_id.clone(), full_chunk_message_header.clone());

            if let Some(message) = message_factor.add_chunk(chunk, &full_chunk_message_header) {
                log::trace!(
                    "[MESSAGE DISAPTCH {:?}  {:?}]",
                    message.message_type,
                    message.time_stamp
                );
                message.dispatch(self, stream).await;
            }

            if self.is_delete {
                break;
            }
        }
        Ok(())
    }
}

pub async fn accpect_rtmp(mut stream: TcpStream) -> Result<()> {
    RtmpCtx::handle_hand_check(&mut stream).await?;
    let mut rtmp_ctx = RtmpCtx::new();
    rtmp_ctx.handle_receive_chunk(&mut stream).await?;
    Ok(())
}
