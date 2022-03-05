use super::{chunk, RtmpCtx};
use crate::util::{async_read_1_byte, async_read_4_byte, AsyncFrom, AR, AW};
use async_trait::async_trait;
use bytes::{buf::Limit, Buf};
use tokio::io::AsyncWriteExt;

#[async_trait]
pub trait Message {
    // 接受消息并执行 相应的逻辑
    async fn dispatch<T>(&self, ctx: &mut RtmpCtx, stream: &mut T)
    where
        T: AR + AW;
    // 发送对应的消息
    async fn send<T>(&self, stream: &mut T)
    where
        T: AW,
    {
    }
}

/// 未知消息
#[derive(Debug, Clone)]
pub struct UnknownMessage;
#[async_trait]
impl Message for UnknownMessage {
    async fn dispatch<Stream>(&self, ctx: &mut RtmpCtx, stream: &mut Stream)
    where
        Stream: AR + AW,
    {
        log::error!("[RECEIVED CHUNK MESSAGE TYPE UNKOWN!]");
        unreachable!()
    }
}

/// 设置消息大小
#[derive(Debug, Clone)]
pub struct SetChunkSize;

#[async_trait]
impl Message for SetChunkSize {
    async fn dispatch<Stream>(&self, ctx: &mut RtmpCtx, stream: &mut Stream)
    where
        Stream: AR + AW,
    {
        let mut bytes = async_read_4_byte(stream).await;
        let chunk_size = bytes.get_u32() << 1 >> 1;
        if chunk_size > 0x7FFFFFFF {
            ctx.chunk_size = 0x7FFFFFFF;
        } else {
            ctx.chunk_size = chunk_size;
        }
    }
}

/// 终止消息
/// 协议控制消息2，终止消息，通知正在等待消息后续块的另一端，可以丢弃通过指定块流接收到的部分数据，块流ID为该消息有效负载。
/// 应用可能在关闭的时候发送该消息，用来表明后面的消息没有必要继续处理了。
#[derive(Debug, Clone)]
pub struct AbortMessage;
#[async_trait]
impl Message for AbortMessage {
    async fn dispatch<Stream>(&self, ctx: &mut RtmpCtx, stream: &mut Stream)
    where
        Stream: AR + AW,
    {
        let mut bytes = async_read_4_byte(stream).await;
        let cs_id = bytes.get_u32();
        // TODO Abort cs_id
    }
}

/// 确认消息
#[derive(Debug, Clone)]
pub struct Acknowledgement;
#[async_trait]
impl Message for Acknowledgement {
    async fn dispatch<Stream>(&self, ctx: &mut RtmpCtx, stream: &mut Stream)
    where
        Stream: AR + AW,
    {
        // TODO 到目前为止收到的字节数

        let sequence_number = 0;
        stream.write_u32(sequence_number).await;
    }
}

/// 视窗大小确认
#[derive(Debug, Clone)]
pub struct WindowAcknowledgement;
#[async_trait]
impl Message for WindowAcknowledgement {
    async fn dispatch<Stream>(&self, ctx: &mut RtmpCtx, stream: &mut Stream)
    where
        Stream: AR + AW,
    {
        let _ = async_read_4_byte(stream).await;
        let ack = Acknowledgement;
        ack.dispatch(ctx, stream).await;
    }
}

#[derive(Debug, Clone)]
/// 设置对等带宽
pub struct SetPeerBandWidth;
#[async_trait]
impl Message for SetPeerBandWidth {
    async fn dispatch<Stream>(&self, ctx: &mut RtmpCtx, stream: &mut Stream)
    where
        Stream: AW + AR,
    {
        let wackl = WindowAcknowledgement;
        wackl.dispatch(ctx, stream).await;
        let limit_type = LimitType::async_from(stream).await;
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
