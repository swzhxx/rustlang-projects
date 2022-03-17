use std::{fs::OpenOptions, io::Write};

use amf::{amf0, Pair};
use async_trait::async_trait;
use bytes::{Buf, BufMut, BytesMut};
use chrono::Local;

use crate::{
    rtmp::protocol::{
        audio_header_map, eventbus_map,
        message::{
            self, Command, CommandMessageAMF020, Message, MessageType, OnMetaData, OnStatus,
        },
        meta_data_map, video_header_map, RtmpCtx,
    },
    util::{AsyncWriteByte, AW},
};

/// 用户控制消息
#[derive(Debug, Clone)]
pub struct UserControlMessage;

impl UserControlMessage {
    pub async fn excute<Writer>(
        chunk_data: &[u8],
        ctx: &mut RtmpCtx,
        writer: &mut Writer,
        message: &Message,
    ) where
        Writer: AW,
    {
        let event_type: EventType = message.try_into().unwrap();
        event_type.dispatch(message, ctx, writer).await;
    }
    pub async fn send<Writer>(
        event_type: EventType,
        event_data: &[u8],
        ctx: &RtmpCtx,
        writer: &mut Writer,
    ) where
        Writer: AW,
    {
        let cs_id = 2;
        // let message_stream_id = 0;
        let event_type_number: u8 = event_type.try_into().unwrap();
        let mut message_body = (event_type_number as u32).to_be_bytes().to_vec();
        message_body.append(&mut event_data.clone().to_vec());
        let message = Message::new(
            cs_id,
            MessageType::USER_CONTROL_MESSAGE(UserControlMessage),
            0,
            0,
            message_body,
        );

        message.async_write_byte(ctx, writer).await;
    }
}

#[async_trait]
pub trait EventExcute {
    async fn excute<Writer>(message: &Message, ctx: &mut RtmpCtx, writer: &mut Writer)
    where
        Writer: AW,
    {
    }
}

#[derive(Debug, Default, Clone)]
pub struct StreamBegin;

#[async_trait]
impl EventExcute for StreamBegin {}

#[derive(Debug, Default, Clone)]
pub struct StreamEOF;

#[async_trait]
impl EventExcute for StreamEOF {}

#[derive(Debug, Default, Clone)]
pub struct SetBufferLength;

#[async_trait]
impl EventExcute for SetBufferLength {
    async fn excute<Writer>(message: &Message, ctx: &mut RtmpCtx, writer: &mut Writer)
    where
        Writer: AW,
    {
        log::trace!("[RECEIVED USER CONTROL MESSAGE] SET BUFFER LENGTH");
        let mut bytes = BytesMut::from_iter(message.message_body[2..].iter());
        let stream_id = 1u32;
        let buffer_length = bytes.get_u32();

        UserControlMessage::send(
            EventType::STREAM_BEGIN(StreamBegin),
            &stream_id.to_be_bytes()[..],
            ctx,
            writer,
        )
        .await;
        // 发送status
        {
            let mut amf_datas = vec![];
            amf_datas.push(amf0::Value::String("onStatus".to_string()));
            amf_datas.push(amf0::Value::Number(0.0));
            amf_datas.push(amf0::Value::Null);
            amf_datas.push(amf0::Value::Object {
                class_name: None,
                entries: vec![
                    Pair {
                        key: "level".to_owned(),
                        value: amf::amf0::Value::String("status".to_owned()),
                    },
                    Pair {
                        key: "code".to_owned(),
                        value: amf::amf0::Value::String("NetStream.Play.Start".to_owned()),
                    },
                    Pair {
                        key: "description".to_owned(),
                        value: amf::amf0::Value::String("Start live".to_owned()),
                    },
                ],
            });

            CommandMessageAMF020::send(Command::OnStatus(OnStatus), &amf_datas, ctx, writer).await;
        }
        {
            // let mut amf_datas = vec![];
            // amf_datas.push(amf::amf0::Value::String("RtmpSampleAccess".to_string()));
            // amf_datas.push(amf::amf0::Value::Boolean(true));
            // amf_datas.push(amf::amf0::Value::Boolean(true));
            // CommandMessageAMF020::send(Command::OnStatus(OnStatus), &amf_datas, ctx, writer).await;
        }
        {
            //send meta Data
            if let Some(meta_data) = meta_data_map().get(ctx.stream_name.as_ref().unwrap()) {
                let mut amf_datas = vec![];
                amf_datas.push(amf0::Value::String("@setDataFrame".to_string()));
                amf_datas.push(amf0::Value::String("onMetaData".to_string()));
                amf_datas.push(amf::amf0::Value::Object {
                    class_name: None,
                    entries: vec![
                        Pair {
                            key: "Server".to_owned(),
                            value: amf::amf0::Value::String("THATS ME".to_owned()),
                        },
                        Pair {
                            key: "width".to_owned(),
                            value: amf::amf0::Value::Number(meta_data.width),
                        },
                        Pair {
                            key: "height".to_owned(),
                            value: amf::amf0::Value::Number(meta_data.height),
                        },
                        Pair {
                            key: "displayWidth".to_owned(),
                            value: amf::amf0::Value::Number(meta_data.width),
                        },
                        Pair {
                            key: "displayHeight".to_owned(),
                            value: amf::amf0::Value::Number(meta_data.height),
                        },
                        Pair {
                            key: "duration".to_owned(),
                            value: amf::amf0::Value::Number(meta_data.duration),
                        },
                        Pair {
                            key: "framerate".to_owned(),
                            value: amf::amf0::Value::Number(meta_data.frame_rate),
                        },
                        Pair {
                            key: "fps".to_owned(),
                            value: amf::amf0::Value::Number(meta_data.frame_rate),
                        },
                        Pair {
                            key: "videocodecid".to_owned(),
                            value: amf::amf0::Value::String(meta_data.video_codec_id.to_string()),
                        },
                        Pair {
                            key: "videodatarate".to_owned(),
                            value: amf::amf0::Value::Number(meta_data.video_data_rate),
                        },
                        Pair {
                            key: "audiocodecid".to_owned(),
                            value: amf::amf0::Value::String(meta_data.audio_codec_id.to_string()),
                        },
                        Pair {
                            key: "audiodatarate".to_owned(),
                            value: amf::amf0::Value::Number(meta_data.audio_data_rate),
                        },
                        Pair {
                            key: "profile".to_owned(),
                            value: amf::amf0::Value::String(Default::default()),
                        },
                        Pair {
                            key: "level".to_owned(),
                            value: amf::amf0::Value::String(Default::default()),
                        },
                        Pair {
                            key: "encoder".to_owned(),
                            value: amf0::Value::String(meta_data.encoder.to_string()),
                        },
                        Pair {
                            key: "stereo".to_owned(),
                            value: amf0::Value::Boolean(meta_data.stereo),
                        },
                        Pair {
                            key: "major_band".to_owned(),
                            value: amf0::Value::String(meta_data.major_brand.to_string()),
                        },
                        Pair {
                            key: "minor_version".to_owned(),
                            value: amf0::Value::String(meta_data.minor_version.to_string()),
                        },
                        Pair {
                            key: "compatible_brands".to_owned(),
                            value: amf0::Value::String(meta_data.compatible_brands.to_string()),
                        },
                    ],
                });
                CommandMessageAMF020::send(
                    Command::OnMetaData(OnMetaData),
                    &amf_datas,
                    ctx,
                    writer,
                )
                .await;
            }
        }
        // ctx.ctx_begin_timestamp = Local::now().timestamp_millis();
        // let src_begin_timestamp = ctx.ctx_begin_timestamp;
        // let begine_time_delta = (0) as u32;
        {
            // // send 视频格式
            // if let Some(msg) = video_header_map().get(ctx.stream_name.as_ref().unwrap()) {
            //     msg.async_write_byte(ctx, writer).await;
            // }
        }
        {
            // // send 音频格式
            // if let Some(msg) = audio_header_map().get(ctx.stream_name.as_ref().unwrap()) {
            //     msg.async_write_byte(ctx, writer).await;
            // }
        }

        {
            // loop
            if let Some(event_bus) = eventbus_map().get(ctx.stream_name.as_ref().unwrap()) {
                let mut receiver = event_bus.register_receiver();
                loop {
                    if let Ok(msg) = receiver.recv().await {
                        // log::trace!("{}   {}  ", msg.time_stamp, begine_time_delta);
                        // msg.time_stamp = msg.time_stamp - begine_time_delta;
                        // log::error!("hahah   {:?}", msg.message_type);
                        // match msg.message_type {
                        //     MessageType::AUDIO_MESSAGE(_) => {
                        //         let mut file = OpenOptions::new()
                        //             .create(true)
                        //             .append(true)
                        //             .open("./asserts/output_send.flv")
                        //             .unwrap();
                        //         let mut bytes = BytesMut::new();
                        //         bytes.put_slice(&msg.message_body[..]);
                        //         match file.write(&bytes) {
                        //             Ok(_) => {}
                        //             Err(err) => log::error!("Err {}", err),
                        //         };
                        //     }
                        //     MessageType::VIDEO_MESSAGE(_) => {
                        //         let mut file = OpenOptions::new()
                        //             .create(true)
                        //             .append(true)
                        //             .open("./asserts/output_send.flv")
                        //             .unwrap();
                        //         let mut bytes = BytesMut::new();
                        //         bytes.put_slice(&msg.message_body[..]);
                        //         match file.write(&bytes) {
                        //             Ok(_) => {}
                        //             Err(err) => log::error!("Err {}", err),
                        //         };
                        //     }
                        //     _ => {}
                        // }

                        // msg.message_stream_id = 0;
                        // log::error!("send start {:?}", msg.message_type);
                        msg.async_write_byte(ctx, writer).await;
                        // log::error!("send end {:?}", msg.message_type);
                    }
                }
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct StreamDry;

#[async_trait]
impl EventExcute for StreamDry {}

#[derive(Debug, Default, Clone)]
pub struct StreamIsRecord;

#[async_trait]
impl EventExcute for StreamIsRecord {}

#[derive(Debug, Default, Clone)]
pub struct PingRequest;

#[async_trait]
impl EventExcute for PingRequest {}

#[derive(Debug, Default, Clone)]
pub struct PingResponse;

#[async_trait]
impl EventExcute for PingResponse {}

#[derive(Debug, Clone)]
pub enum EventType {
    STREAM_BEGIN(StreamBegin),
    STREAM_EOF(StreamEOF),
    STREAM_DRY(StreamDry),
    SET_BUFFER_LENGTH(SetBufferLength),
    STREAM_IS_RECORD(StreamIsRecord),
    PING_REQUEST(PingRequest),
    PING_RESPONSE(PingResponse),
}

impl TryFrom<&Message> for EventType {
    type Error = String;
    fn try_from(message: &Message) -> Result<EventType, String> {
        let message_type = &message.message_type;
        match message_type {
            MessageType::USER_CONTROL_MESSAGE(_) => {
                let message_data = &message.message_body;
                let mut bytes = BytesMut::from_iter(&message_data[0..2]);
                let event_type_num = bytes.get_u16();
                match event_type_num {
                    0 => Ok(EventType::STREAM_BEGIN(StreamBegin)),
                    1 => Ok(EventType::STREAM_EOF(StreamEOF)),
                    2 => Ok(EventType::STREAM_DRY(StreamDry)),
                    3 => Ok(EventType::SET_BUFFER_LENGTH(SetBufferLength)),
                    4 => Ok(EventType::STREAM_IS_RECORD(StreamIsRecord)),
                    5 => Ok(EventType::PING_REQUEST(PingRequest)),
                    6 => Ok(EventType::PING_RESPONSE(PingResponse)),
                    _ => return Err("message to eventype error".to_string()),
                }
            }
            _ => return Err("message to eventype error".to_string()),
        }
    }
}

impl TryInto<u8> for EventType {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<u8, Self::Error> {
        match self {
            EventType::STREAM_BEGIN(_) => Ok(0),
            EventType::STREAM_EOF(_) => Ok(1),
            EventType::STREAM_DRY(_) => Ok(2),
            EventType::SET_BUFFER_LENGTH(_) => Ok(3),
            EventType::STREAM_IS_RECORD(_) => Ok(4),
            EventType::PING_REQUEST(_) => Ok(5),
            EventType::PING_RESPONSE(_) => Ok(6),
        }
    }
}

impl EventType {
    async fn dispatch<Writer>(&self, message: &Message, ctx: &mut RtmpCtx, writer: &mut Writer)
    where
        Writer: AW,
    {
        match self {
            EventType::STREAM_BEGIN(_) => {}
            EventType::STREAM_EOF(_) => {}
            EventType::STREAM_DRY(_) => {}
            EventType::SET_BUFFER_LENGTH(event) => {
                SetBufferLength::excute(message, ctx, writer).await
            }
            EventType::STREAM_IS_RECORD(_) => {}
            EventType::PING_REQUEST(_) => {}
            EventType::PING_RESPONSE(_) => {}
        }
    }
}
