use crate::{
    rtmp::protocol::RtmpCtx,
    util::{async_read_1_byte, AsyncFrom, AR, AW},
};
use async_trait::async_trait;
use bytes::{buf::Limit, Buf, BytesMut};
use tokio::io::{AsyncRead, AsyncWriteExt};

use super::Message;

/// 未知消息
#[derive(Debug, Clone)]
pub struct UnknownMessage;

/// 设置消息大小
#[derive(Debug, Clone)]
pub struct SetChunkSize;
impl SetChunkSize {
    async fn excute(chunk_data: &[u8], ctx: &mut RtmpCtx) {
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
}

/// 终止消息
/// 协议控制消息2，终止消息，通知正在等待消息后续块的另一端，可以丢弃通过指定块流接收到的部分数据，块流ID为该消息有效负载。
/// 应用可能在关闭的时候发送该消息，用来表明后面的消息没有必要继续处理了。
#[derive(Debug, Clone)]
pub struct AbortMessage;
impl AbortMessage {
    async fn excute(chunk_data: &[u8], ctx: &mut RtmpCtx) {
        let mut bytes = BytesMut::from_iter(chunk_data.iter());
        let abort_id = bytes.get_u32();
        ctx.abort_chunk_id = Some(abort_id);
    }
}

/// 确认消息
#[derive(Debug, Clone)]
pub struct Acknowledgement;
impl Acknowledgement {
    async fn excute<Writer>(chunk_data: &[u8], ctx: &mut RtmpCtx, writer: &mut Writer)
    where
        Writer: AW,
    {
        let sequence_number = ctx.reve_bytes as u32;
        writer.write_u32(sequence_number).await;
    }
}

/// 视窗大小确认
#[derive(Debug, Clone)]
pub struct WindowAcknowledgement;
impl WindowAcknowledgement {
    async fn excute<Writer>(chunk_data: &[u8], ctx: &mut RtmpCtx, writer: &mut Writer) {
        let mut bytes = BytesMut::from_iter(chunk_data.iter());
        let ack_window_size = bytes.get_u32();
        // TODO , 我还需要做什么?
    }
}

#[derive(Debug, Clone)]
/// 设置对等带宽
pub struct SetPeerBandWidth;
impl SetPeerBandWidth {
    async fn excute<Writer>(chunk_data: &[u8], ctx: &mut RtmpCtx, writer: &mut Writer) {
        WindowAcknowledgement::excute(&chunk_data[0..4], ctx, writer);
        let limit = chunk_data[5];
        let limit_type = LimitType::try_from(limit);
        // TODO , 我还需要做什么?
    }
}

#[derive(Debug, Clone)]
pub enum LimitType {
    Hard,
    Soft,
    Dynamic,
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

/// 用户控制消息
#[derive(Debug, Clone)]
pub struct UserControlMessage;

impl UserControlMessage {
    async fn excute() {
        todo!()
    }
}

// todo! start ---- AMF ENCODE ---

///  指令消息(20,17)
/// 指令消息在客户端和服务端之间传递AMF编码的指令，
/// 消息类型20代表AMF0编码，消息类型17代表AMF3编码。
/// 发送这些消息来完成连接、创建流、发布、播放、暂停等操作。
/// 像状态、结果这样的指令消息，用于通知发送方请求的指令状态。
/// 一条指令消息由指令名、事务ID和包含相关参数的指令对象。
/// 客户端或服务端还可以通过指令消息来实现远程过程调用(RPC)。
#[derive(Debug, Clone)]
pub struct CommandMessage20;

#[derive(Debug, Clone)]
pub struct CommandMessage17;

///数据消息(18,15)
///客户端或服务端通过该消息来发送元数据或其他用户数据。
/// 元数据包括数据(音频、视频)的创建时间、时长、主题等详细信息。
/// 消息类型18代表AMF0编码，消息类型15代表AMF3编码

#[derive(Debug, Clone)]
pub struct DataMessage18;

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

#[derive(Debug, Clone)]
pub struct VideoMessage;

#[derive(Debug, Clone)]
pub struct AggregrateMessage;

impl AggregrateMessage {
    async fn excute() {}
}
#[derive(Debug, Clone)]
pub enum MessageType {
    UNKOWN,
    SET_CHUNK_SIZE(SetChunkSize),
    ABORT_MESSAGE(AbortMessage),
    ACKNOWLEDGEMENT(Acknowledgement),
    WINDOW_ACKNOWLEDGEMENT(WindowAcknowledgement),
    SET_PEER_BANDWIDTH(SetPeerBandWidth),
    USER_CONTROL_MESSAGE(UserControlMessage),
    COMMAND_MESSAGE_20(CommandMessage20),
    COMMAND_MESSAGE_17(CommandMessage17),
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
            20 => Self::COMMAND_MESSAGE_20(CommandMessage20),
            17 => Self::COMMAND_MESSAGE_17(CommandMessage17),
            _ => todo!(),
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
            _ => todo!(),
        }
    }
}

impl MessageType {
    pub async fn dispatch<'a, T>(&self, chunk_data: &'a [u8], ctx: &'a mut RtmpCtx, stream: &mut T)
    where
        T: AR + AW,
    {
        match self {
            MessageType::UNKOWN => todo!(),
            MessageType::SET_CHUNK_SIZE(message) => {
                SetChunkSize::excute(chunk_data, ctx).await;
            }
            MessageType::ABORT_MESSAGE(_) => {
                AbortMessage::excute(chunk_data, ctx).await;
            }
            MessageType::ACKNOWLEDGEMENT(_) => {
                Acknowledgement::excute(chunk_data, ctx, stream).await;
            }
            MessageType::WINDOW_ACKNOWLEDGEMENT(_) => {
                WindowAcknowledgement::excute(chunk_data, ctx, stream).await;
            }
            MessageType::SET_PEER_BANDWIDTH(_) => {
                SetPeerBandWidth::excute(chunk_data, ctx, stream).await
            }
            _ => todo!(),
        }
    }
}
