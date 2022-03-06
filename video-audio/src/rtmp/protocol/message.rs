use super::{chunk, RtmpCtx};
use crate::util::{async_read_1_byte, AsyncFrom, AR, AW};
use async_trait::async_trait;
use bytes::{buf::Limit, Buf};
use tokio::io::{AsyncRead, AsyncWriteExt};

/// 未知消息
#[derive(Debug, Clone)]
pub struct UnknownMessage;

/// 设置消息大小
#[derive(Debug, Clone)]
pub struct SetChunkSize;
impl SetChunkSize {
    fn excute(chunk_data: &[u8], ctx: &mut RtmpCtx) {}
}

/// 终止消息
/// 协议控制消息2，终止消息，通知正在等待消息后续块的另一端，可以丢弃通过指定块流接收到的部分数据，块流ID为该消息有效负载。
/// 应用可能在关闭的时候发送该消息，用来表明后面的消息没有必要继续处理了。
#[derive(Debug, Clone)]
pub struct AbortMessage;

/// 确认消息
#[derive(Debug, Clone)]
pub struct Acknowledgement;

/// 视窗大小确认
#[derive(Debug, Clone)]
pub struct WindowAcknowledgement;

#[derive(Debug, Clone)]
/// 设置对等带宽
pub struct SetPeerBandWidth;

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

#[derive(Debug, Clone)]
pub enum MessageType {
    UNKOWN,
    SET_CHUNK_SIZE(SetChunkSize),
    ABORT_MESSAGE(AbortMessage),
    ACKNOWLEDGEMENT(Acknowledgement),
    WINDOW_ACKNOWLEDGEMENT(WindowAcknowledgement),
    SET_PEER_BANDWIDTH(SetPeerBandWidth),
}

impl From<u8> for MessageType {
    fn from(_: u8) -> Self {
        todo!()
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
                SetChunkSize::excute(chunk_data, ctx);
            }
            MessageType::ABORT_MESSAGE(_) => todo!(),
            MessageType::ACKNOWLEDGEMENT(_) => todo!(),
            MessageType::WINDOW_ACKNOWLEDGEMENT(_) => todo!(),
            MessageType::SET_PEER_BANDWIDTH(_) => todo!(),
            _ => todo!(),
        }
    }
}
