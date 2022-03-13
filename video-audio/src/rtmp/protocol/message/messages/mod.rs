use std::clone;

use crate::{
    rtmp::protocol::{
        audio_header_map, eventbus_map, meta_data_map, video_header_map, RtmpCtx, RtmpMetaData,
    },
    util::{async_read_1_byte, read_all_amf_value, AsyncFrom, AsyncWriteByte, AR, AW},
};
use async_trait::async_trait;
use bytes::{buf::Limit, Buf, BytesMut};
use tokio::io::{AsyncRead, AsyncWriteExt};

use super::Message;
mod user_control_message;
pub use user_control_message::*;

mod command_message;
pub use command_message::*;

/// RTMP块流用消息类型ID 1，2，3，5和6来作为协议控制消息，这些消息包含RTMP块流协议所需要的信息。
/// 这些协议控制消息必须用 0作为消息流ID(控制流ID)，并在ID为2的块流中发送。
/// 协议控制消息收到后立即生效，它们的时间戳信息是被忽略的。

/// 未知消息
#[derive(Debug, Clone)]
pub struct UnknownMessage;

/// 设置消息大小
#[derive(Debug, Clone)]
pub struct SetChunkSize;
impl SetChunkSize {
    pub async fn excute(chunk_data: &[u8], ctx: &mut RtmpCtx) {
        let mut bytes = BytesMut::from_iter(chunk_data.iter());
        let chunk_size = bytes.get_u32();
        let chunk_size = chunk_size << 1 >> 1;
        let chunk_size = {
            if chunk_size > 0x7FFFFFFF {
                0x7FFFFFFF
            } else {
                chunk_size
            }
        };
        ctx.chunk_size = chunk_size
    }

    pub async fn send<Writer>(chunk_size: u32, ctx: &mut RtmpCtx, writer: &mut Writer)
    where
        Writer: AW,
    {
        ctx.chunk_size = chunk_size;
        let message = Message::new(
            2,
            MessageType::SET_CHUNK_SIZE(Self),
            0,
            0,
            chunk_size.to_be_bytes().to_vec(),
        );
        message.async_write_byte(writer).await
    }
}

/// 终止消息
/// 协议控制消息2，终止消息，通知正在等待消息后续块的另一端，可以丢弃通过指定块流接收到的部分数据，块流ID为该消息有效负载。
/// 应用可能在关闭的时候发送该消息，用来表明后面的消息没有必要继续处理了。
#[derive(Debug, Clone)]
pub struct AbortMessage;
impl AbortMessage {
    pub async fn excute(chunk_data: &[u8], ctx: &mut RtmpCtx) {
        let mut bytes = BytesMut::from_iter(chunk_data.iter());
        let abort_id = bytes.get_u32();
        ctx.abort_chunk_id = Some(abort_id);
    }
}

/// 确认消息
#[derive(Debug, Clone)]
pub struct Acknowledgement;
impl Acknowledgement {
    pub async fn excute<Writer>(chunk_data: &[u8], ctx: &mut RtmpCtx, writer: &mut Writer)
    where
        Writer: AW,
    {
        // let sequence_number = ctx.reve_bytes as u32;
    }

    async fn send<Writer>(ctx: &mut RtmpCtx, writer: &mut Writer)
    where
        Writer: AW,
    {
        let sequence_number = ctx.reve_bytes as u32;
        // writer.write_u32(sequence_number).await;
        let message = Message::new(
            2,
            MessageType::ACKNOWLEDGEMENT(Self),
            0,
            2,
            sequence_number.to_be_bytes().to_vec(),
        );
        message.async_write_byte(writer).await;
    }
}

/// 视窗大小确认
#[derive(Debug, Clone)]
pub struct WindowAcknowledgement;
impl WindowAcknowledgement {
    pub async fn excute<Writer>(chunk_data: &[u8], ctx: &mut RtmpCtx, writer: &mut Writer)
    where
        Writer: AW,
    {
        let mut bytes = BytesMut::from_iter(chunk_data.iter());
        let ack_window_size = bytes.get_u32();

        // TODO ,在接收到对方的size后 我还需要做什么?

        // 回应确认消息
        Acknowledgement::send(ctx, writer).await;
    }

    pub async fn send<Writer>(ack_window_size: u32, ctx: &mut RtmpCtx, writer: &mut Writer)
    where
        Writer: AW,
    {
        let message = Message::new(
            2,
            MessageType::WINDOW_ACKNOWLEDGEMENT(WindowAcknowledgement),
            0,
            0,
            ack_window_size.to_be_bytes().to_vec(),
        );

        message.async_write_byte(writer).await
    }
}

#[derive(Debug, Clone)]
/// 设置对等带宽
pub struct SetPeerBandWidth;
impl SetPeerBandWidth {
    pub async fn excute<Writer>(chunk_data: &[u8], ctx: &mut RtmpCtx, writer: &mut Writer)
    where
        Writer: AW,
    {
        let mut bytes = BytesMut::from_iter(chunk_data.iter());
        let ack_window_size = bytes.get_u32();
        let limit = chunk_data[5];
        let limit_type = LimitType::try_from(limit);

        // TODO , 我还需要做什么?
    }

    pub async fn send<Writer>(
        ack_window_size: u32,
        limit_type: LimitType,
        _ctx: &mut RtmpCtx,
        writer: &mut Writer,
    ) where
        Writer: AW,
    {
        // let limit_type = LimitType::try_from(limit_type).unwrap();
        let mut chunk_data = ack_window_size.to_be_bytes().to_vec();
        let limit_type: u8 = limit_type.try_into().unwrap();
        chunk_data.push(limit_type);
        let message = Message::new(
            2,
            MessageType::SET_PEER_BANDWIDTH(SetPeerBandWidth),
            0,
            0,
            chunk_data,
        );

        message.async_write_byte(writer).await;
    }
}

#[derive(Debug, Clone)]
pub enum LimitType {
    Hard = 0,
    Soft = 1,
    Dynamic = 2,
}

impl TryFrom<u8> for LimitType {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(LimitType::Hard),
            1 => Ok(LimitType::Soft),
            2 => Ok(LimitType::Dynamic),
            _ => Err("Limit Type TryFrom Error".to_string()),
        }
    }
}

impl TryFrom<LimitType> for u8 {
    type Error = String;

    fn try_from(value: LimitType) -> Result<Self, Self::Error> {
        match value {
            LimitType::Hard => Ok(0),
            LimitType::Soft => Ok(1),
            LimitType::Dynamic => Ok(2),
            _ => Err("LimitType to u8 error".to_string()),
        }
    }
}

#[async_trait]
impl AsyncFrom for LimitType {
    async fn async_from<Reader>(reader: &mut Reader) -> Self
    where
        Reader: AR,
    {
        let mut bytes = async_read_1_byte(reader).await;
        let value = bytes.get_u8();
        let limit_type = value.try_into();
        limit_type.unwrap()
    }
}

// todo! start ---- AMF ENCODE ---

///数据消息(18,15)
///客户端或服务端通过该消息来发送元数据或其他用户数据。
/// 元数据包括数据(音频、视频)的创建时间、时长、主题等详细信息。
/// 消息类型18代表AMF0编码，消息类型15代表AMF3编码

#[derive(Debug, Clone)]
pub struct DataMessage18;

impl DataMessage18 {
    pub async fn excute<Writer>(chunk_data: &[u8], ctx: &mut RtmpCtx, writer: &mut Writer)
    where
        Writer: AW,
    {
        let values = read_all_amf_value(chunk_data);
        if let Some(values) = values {
            let command = (&values[0]).try_as_str().unwrap();
            if command == "@setDataFrame" {
                if let Ok(meta_data) = RtmpMetaData::try_from(&values[2]) {
                    meta_data_map().insert(ctx.stream_name.as_ref().unwrap().clone(), meta_data);
                }
            }
        } else {
        }
    }
}

#[derive(Debug, Clone)]
pub struct DataMessage15;

/// 共享对象消息(19,16)
/// 共享对象是一个在多个客户端、示例之间进行同步的Flash对象(键值对集合)。
/// 消息类型19代表AMF0编码，消息类型16代表AMF3编码。
/// 每个消息都可以包含多个事件
///
#[derive(Debug, Clone)]
pub struct SharedObjectMessage19;

#[derive(Clone, Debug)]
pub struct SharedObjectMessage16;

// todo! end ---- AMF ENCODE ---

#[derive(Debug, Clone)]
pub struct AudioMessage;

impl AudioMessage {
    pub async fn excute<Writer>(
        chunk_data: &[u8],
        ctx: &mut RtmpCtx,
        _writer: &mut Writer,
        message: &Message,
    ) where
        Writer: AW,
    {
        audio_header_map().insert(ctx.stream_name.as_ref().unwrap().clone(), message.clone());
        log::trace!(
            "[REIVED AUDIO MESSAGE] -> AUDIO DATA LEN {}",
            chunk_data.len()
        );
        // todo 将音频文件保存在本地,为后续hlv做准备
        if let Some(eventbus) = eventbus_map().get(ctx.stream_name.as_ref().unwrap()) {
            eventbus.publish(message.clone()).await
        }
    }
}

#[derive(Debug, Clone)]
pub struct VideoMessage;

impl VideoMessage {
    pub async fn excute<Writer>(
        chunk_data: &[u8],
        ctx: RtmpCtx,
        _writer: &mut Writer,
        message: &Message,
    ) where
        Writer: AW,
    {
        video_header_map().insert(ctx.stream_name.as_ref().unwrap().clone(), message.clone());
        log::trace!(
            "[REIVED AUDIO MESSAGE] -> VIDEO DATA LEN {}",
            chunk_data.len()
        );
        // todo 将视频文件保存在本地,为后续hlv做准备
        if let Some(eventbus) = eventbus_map().get(ctx.stream_name.as_ref().unwrap()) {
            eventbus.publish(message.clone()).await
        }
    }
}

#[derive(Debug, Clone)]
pub struct AggregrateMessage;

impl AggregrateMessage {
    async fn excute() {}
}

/// Message Type 数据类型

#[derive(Debug, Clone)]
pub enum MessageType {
    UNKOWN,
    SET_CHUNK_SIZE(SetChunkSize),
    ABORT_MESSAGE(AbortMessage),
    ACKNOWLEDGEMENT(Acknowledgement),
    WINDOW_ACKNOWLEDGEMENT(WindowAcknowledgement),
    AUDIO_MESSAGE(AudioMessage),
    VIDEO_MESSAGE(VideoMessage),
    SET_PEER_BANDWIDTH(SetPeerBandWidth),
    USER_CONTROL_MESSAGE(UserControlMessage),
    COMMAND_MESSAGE_AMF0_20(CommandMessageAMF020),
    COMMAND_MESSAGE_AMF3_17(CommandMessageAMF317),
    DATA_MESSAGE_18(DataMessage18),
    DATA_MESSAGE_15(DataMessage15),
    SHARED_OBJECT_MESSAGE_19(SharedObjectMessage19),
    SHARED_OBJECT_MESSAGE_16(SharedObjectMessage16),
}

impl From<u8> for MessageType {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::SET_CHUNK_SIZE(SetChunkSize),
            2 => Self::ABORT_MESSAGE(AbortMessage),
            4 => Self::USER_CONTROL_MESSAGE(UserControlMessage),
            5 => Self::WINDOW_ACKNOWLEDGEMENT(WindowAcknowledgement),
            6 => Self::SET_PEER_BANDWIDTH(SetPeerBandWidth),
            8 => Self::AUDIO_MESSAGE(AudioMessage),
            9 => Self::VIDEO_MESSAGE(VideoMessage),
            20 => Self::COMMAND_MESSAGE_AMF0_20(CommandMessageAMF020),
            17 => Self::COMMAND_MESSAGE_AMF3_17(CommandMessageAMF317),
            18 => Self::DATA_MESSAGE_18(DataMessage18),
            15 => Self::DATA_MESSAGE_15(DataMessage15),
            19 => Self::SHARED_OBJECT_MESSAGE_19(SharedObjectMessage19),
            16 => Self::SHARED_OBJECT_MESSAGE_16(SharedObjectMessage16),
            _ => {
                log::error!("[From MessageType {}]", value);
                todo!()
            }
        }
    }
}

impl Into<u8> for MessageType {
    fn into(self) -> u8 {
        match self {
            MessageType::UNKOWN => todo!(),
            MessageType::SET_CHUNK_SIZE(_) => 1,
            MessageType::ABORT_MESSAGE(_) => 2,
            MessageType::ACKNOWLEDGEMENT(_) => 3,
            MessageType::WINDOW_ACKNOWLEDGEMENT(_) => 5,
            MessageType::SET_PEER_BANDWIDTH(_) => 6,
            MessageType::USER_CONTROL_MESSAGE(_) => 4,
            MessageType::AUDIO_MESSAGE(_) => 8,
            MessageType::VIDEO_MESSAGE(_) => 9,
            MessageType::COMMAND_MESSAGE_AMF0_20(_) => 20,
            MessageType::COMMAND_MESSAGE_AMF3_17(_) => 17,
            MessageType::DATA_MESSAGE_18(_) => 18,
            MessageType::DATA_MESSAGE_15(_) => 15,
            MessageType::SHARED_OBJECT_MESSAGE_19(_) => 19,
            MessageType::SHARED_OBJECT_MESSAGE_16(_) => 16,

            _ => todo!(),
        }
    }
}
