use std::{collections::HashMap, pin::Pin};

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
mod message;

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

pub fn meta_data_map() -> &'static DashMap<String, Message> {
    static INSTANCE: OnceCell<DashMap<String, Message>> = OnceCell::new();
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
                let result = Chunk::async_read_chunk(&mut effect_reader, self).await;
                self.reve_bytes += effect_reader.get_readed_bytes_num();
                log::info!("[RECEIVED MESSAGE TYPE] -> {:?}", result.1.message_type);
                result
            };
            self.last_full_chunk_message_header
                .insert(chunk.cs_id.clone(), full_chunk_message_header.clone());

            if let Some(message) = message_factor.add_chunk(chunk, &full_chunk_message_header) {
                log::trace!("[MESSAGE DISAPTCH {:?}]", message.message_type);
                message
                    .message_type
                    .dispatch(&message.message_body, self, stream)
                    .await;
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
