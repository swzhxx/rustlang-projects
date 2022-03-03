use std::marker::PhantomData;

use crate::util::{
    async_read_1_byte, async_read_2_byte, async_read_3_byte, async_read_4_byte,
    async_read_num_byte, AsyncFrom, AR,
};
use async_trait::async_trait;
use bytes::{Buf, BytesMut};

/// 当Basic Header为1个字节时，CSID占6位，6位最多可以表示64个数，因此这种情况下CSID在［0，63］之间，其中用户可自定义的范围为［3，63］。
/// 当Basic Header为2个字节时，CSID占14位，此时协议将与chunk type所在字节的其他位都置为0，剩下的一个字节来表示CSID－64，这样共有8个二进制位来存储CSID，8位可以表示［0，255］共256个数，因此这种情况下CSID在［64，319］，其中319=255+64。
/// 当Basic Header为3个字节时，CSID占22位，此时协议将［2，8］字节置为1，余下的16个字节表示CSID－64，这样共有16个位来存储CSID，16位可以表示［0，65535］共65536个数，因此这种情况下CSID在［64，65599］，其中65599=65535+64，需要注意的是，Basic Header是采用小端存储的方式，越往后的字节数量级越高，因此通过这3个字节每一位的值来计算CSID时，应该是:<第三个字节的值>x256+<第二个字节的值>+64

struct Chunk {
    pub cs_id: u32,
    pub chunk_header: ChunkHeader,
    pub message_data: Vec<u8>,
    pub extend_stamp: u32,
}

impl Chunk {
    pub fn get_real_time_stamp(&self) -> i64 {
        let timestamp = match &self.chunk_header {
            ChunkHeader::ChunkHeader11(chunk) => chunk.time_stamp,
            ChunkHeader::ChunkHeader7(chunk) => chunk.time_stamp_delta,
            ChunkHeader::ChunkHeader3(chunk) => chunk.time_stamp_delta,
            ChunkHeader::ChunkHeader0(_) => todo!(),
        };
        if self.extend_stamp > 0 {
            (((self.extend_stamp as u64) << 24) | 0xFFFFFF00000000) as i64
        } else {
            timestamp as i64
        }
    }
}

#[async_trait]
impl AsyncFrom for Chunk {
    async fn async_from<Reader>(reader: &mut Reader) -> Self
    where
        Reader: AR,
    {
        let mut byte = async_read_1_byte(reader).await;
        let one = byte.get_u8();
        let fmt = one >> 6;
        let mut cs_id = 0 as u32;
        let chunk_header = match fmt {
            0 => {
                cs_id = (one << 2 >> 2) as u32;
                ChunkHeader::ChunkHeader11(ChunkHeader11::async_from(reader).await)
            }
            1 => {
                cs_id = (async_read_1_byte(reader).await.get_u8() as u32) + 64;
                ChunkHeader::ChunkHeader7(ChunkHeader7::async_from(reader).await)
            }
            2 => {
                let mut byte = async_read_2_byte(reader).await;
                cs_id = byte.get_u16_le() as u32 + 64;
                ChunkHeader::ChunkHeader3(ChunkHeader3::async_from(reader).await)
            }
            3 => ChunkHeader::ChunkHeader0(ChunkHeader0),
            _ => {
                unreachable!()
            }
        };

        let timestamp = match &chunk_header {
            ChunkHeader::ChunkHeader11(chunk) => chunk.time_stamp,
            ChunkHeader::ChunkHeader7(chunk) => chunk.time_stamp_delta,
            ChunkHeader::ChunkHeader3(chunk) => chunk.time_stamp_delta,
            ChunkHeader::ChunkHeader0(_) => {
                // todo  this is error! 该类型需要比较最近相同的ChunkId的块是否存在timestamp
                0
            }
        };

        let mut extend_stamp = 0;
        if timestamp == 16777215 {
            extend_stamp = async_read_4_byte(reader).await.get_u32();
        }

        let message_data = vec![];
        Self {
            cs_id,
            chunk_header,
            message_data,
            extend_stamp: extend_stamp,
        }
    }
}

pub enum ChunkHeader {
    ChunkHeader11(ChunkHeader11),
    ChunkHeader7(ChunkHeader7),
    ChunkHeader3(ChunkHeader3),
    ChunkHeader0(ChunkHeader0),
}

#[derive(Debug)]
pub struct ChunkHeader11 {
    // timestamp_delta:
    time_stamp: u32,
    message_length: u32,
    message_type: MessageType,
    message_stream_id: u32,
}

#[async_trait]
impl AsyncFrom for ChunkHeader11 {
    async fn async_from<Reader>(reader: &mut Reader) -> Self
    where
        Reader: AR,
    {
        let mut bytes = async_read_num_byte(reader, 11).await;
        let b32 = bytes.get_u32();
        let time_stamp = b32 >> 8;
        let reminder = (b32 << 24 >> 24) as u8;

        let r_message_length = bytes.get_u16().to_be_bytes();
        let mut rbytes =
            BytesMut::from_iter([0u8, reminder, r_message_length[0], r_message_length[1]].iter());
        let message_length = rbytes.get_u32();
        let message_type_id = bytes.get_u8();
        // TODO Message Type
        let message_type = MessageType::UNKOWN;
        let mut message_stream_id_bytes = bytes.get(0..3).unwrap().to_vec();
        message_stream_id_bytes.insert(0, 0u8);
        let message_stream_id = BytesMut::from_iter(message_stream_id_bytes.iter()).get_u32();

        Self {
            time_stamp,
            message_length,
            message_type,
            message_stream_id,
        }
    }
}

#[derive(Debug)]
pub struct ChunkHeader7 {
    time_stamp_delta: u32,
    message_length: u32,
    message_type: MessageType,
}

#[async_trait]
impl AsyncFrom for ChunkHeader7 {
    async fn async_from<Reader>(reader: &mut Reader) -> Self
    where
        Reader: AR,
    {
        let mut b4 = async_read_4_byte(reader).await;
        let b4 = b4.get_u32();
        let time_stamp_delta = b4 >> 8;
        let message_length_header = b4 << 24;
        let message_reminder = async_read_2_byte(reader).await.get_u16();
        let message_length = message_length_header << 16 + message_reminder;

        let message_type_id = async_read_1_byte(reader).await.get_u8();
        // TODO
        let message_type = MessageType::UNKOWN;
        Self {
            time_stamp_delta,
            message_length,
            message_type,
        }
    }
}

#[derive(Debug)]
pub struct ChunkHeader3 {
    time_stamp_delta: u32,
}

#[async_trait]
impl AsyncFrom for ChunkHeader3 {
    async fn async_from<Reader>(reader: &mut Reader) -> Self
    where
        Reader: AR,
    {
        let mut b3 = async_read_3_byte(reader).await;
        let b1 = b3.get_u8();
        let b2 = b3.get_u16();

        let time_stamp_delta = (b1 as u32) << 16 + b2;
        Self { time_stamp_delta }
    }
}

#[derive(Debug)]
struct ChunkHeader0;

#[derive(Debug)]
enum MessageType {
    UNKOWN = 0,
}

impl From<u8> for MessageType {
    fn from(_: u8) -> Self {
        todo!()
    }
}
