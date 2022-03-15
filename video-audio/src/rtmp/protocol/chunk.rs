use std::marker::PhantomData;

use crate::util::{
    async_read_1_byte, async_read_2_byte, async_read_3_byte, async_read_4_byte,
    async_read_num_byte, AsyncFrom, AsyncWriteByte, AR, AW,
};
use async_trait::async_trait;
use bytes::{Buf, BufMut, BytesMut};
use tokio::io::AsyncWriteExt;

use super::{message::MessageType, RtmpCtx};

/// 当Basic Header为1个字节时，CSID占6位，6位最多可以表示64个数，因此这种情况下CSID在［0，63］之间，其中用户可自定义的范围为［3，63］。
/// 当Basic Header为2个字节时，CSID占14位，此时协议将与chunk type所在字节的其他位都置为0，剩下的一个字节来表示CSID－64，这样共有8个二进制位来存储CSID，8位可以表示［0，255］共256个数，因此这种情况下CSID在［64，319］，其中319=255+64。
/// 当Basic Header为3个字节时，CSID占22位，此时协议将［2，8］字节置为1，余下的16个字节表示CSID－64，这样共有16个位来存储CSID，16位可以表示［0，65535］共65536个数，因此这种情况下CSID在［64，65599］，其中65599=65535+64，需要注意的是，Basic Header是采用小端存储的方式，越往后的字节数量级越高，因此通过这3个字节每一位的值来计算CSID时，应该是:<第三个字节的值>x256+<第二个字节的值>+64

#[derive(Debug)]
pub struct Chunk {
    pub cs_id: u32,
    pub chunk_header: ChunkMessageHeader,
    pub message_data: Vec<u8>,
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            cs_id: Default::default(),
            chunk_header: ChunkMessageHeader::ChunkMessageHeader0(ChunkMessageHeader0),
            message_data: Default::default(),
        }
    }
}

impl Chunk {
    pub async fn async_read_chunk<Reader>(
        reader: &mut Reader,
        ctx: &RtmpCtx,
    ) -> anyhow::Result<(Self, FullChunkMessageHeader)>
    where
        Reader: AR,
    {
        let mut byte = async_read_1_byte(reader).await?;
        let one = byte.get_u8();

        let fmt = one >> 6;
        log::info!("[RECEIVE CHUNK fmt] {}", fmt);
        let mut cs_id = (one << 2 >> 2) as u32;
        if cs_id == 0 {
            // 如果低6位的字节为0 ， 则再读取一个字节 cs_id = 第二个字节的值 + 64;
            let mut byts = async_read_1_byte(reader).await?;
            let num = byts.get_u8();
            cs_id = num as u32 + 64;
        } else if cs_id == 1 {
            let mut bytes = async_read_1_byte(reader).await?;
            let num_1 = bytes.get_u8();
            let mut bytes = async_read_1_byte(reader).await?;
            let num_2 = bytes.get_u8();
            cs_id = num_2 as u32 * 255 + num_1 as u32 + 64;
        }
        let chunk_message_header = match fmt {
            0 => ChunkMessageHeader::ChunkMessageHeader11(
                ChunkMessageHeader11::async_from(reader).await,
            ),
            1 => ChunkMessageHeader::ChunkMessageHeader7(
                ChunkMessageHeader7::async_from(reader).await,
            ),
            2 => ChunkMessageHeader::ChunkMessageHeader3(
                ChunkMessageHeader3::async_from(reader).await,
            ),
            3 => ChunkMessageHeader::ChunkMessageHeader0(ChunkMessageHeader0),
            _ => {
                unreachable!()
            }
        };

        // TODO extend timestamp 解析，这将是一个错误 , 但是现在我们不考虑
        let mut will_return_full_chunk_message_header: FullChunkMessageHeader;
        // ChunMessagekHeader的解析
        let last_full_chunk_message_header = ctx.last_full_chunk_message_header.get(&cs_id);

        let mut will_read_message_length: u32 = 0;

        let calc_read_size = |message_length| {
            let will_read_size = message_length as i64 - ctx.chunk_size as i64;
            if will_read_size > 0 {
                ctx.chunk_size
            } else {
                message_length
            }
        };
        let mut_reminder_message_length =
            |chunk_message_header: &mut FullChunkMessageHeader, will_read_size: u32| {
                let reminder = {
                    if chunk_message_header.reminder_message_length == 0 {
                        chunk_message_header.message_length as i64 - will_read_size as i64
                    } else {
                        chunk_message_header.reminder_message_length as i64 - will_read_size as i64
                    }
                };
                if reminder < 0 {
                    chunk_message_header.reminder_message_length = 0
                } else {
                    chunk_message_header.reminder_message_length = reminder as u32
                }
            };

        if last_full_chunk_message_header.is_none() {
            will_read_message_length = match &chunk_message_header {
                ChunkMessageHeader::ChunkMessageHeader11(chunk_message_header) => {
                    will_return_full_chunk_message_header = chunk_message_header.into();
                    let read_size =
                        calc_read_size(will_return_full_chunk_message_header.message_length);
                    mut_reminder_message_length(
                        &mut will_return_full_chunk_message_header,
                        read_size,
                    );
                    read_size
                }

                _ => {
                    log::error!(
                        "[CHUNK ERROR] -> fmt {:?} ,cs_id {:?}, message_header {:?} , receive_chunk_ids {:?} , chunk_size {:?}",
                        fmt,
                        cs_id,
                        chunk_message_header,
                        ctx.last_full_chunk_message_header.keys(),
                        ctx.chunk_size
                    );
                    unreachable!()
                }
            }
        } else {
            let last_full_chunk_message_header = last_full_chunk_message_header.unwrap();
            will_read_message_length = match &chunk_message_header {
                ChunkMessageHeader::ChunkMessageHeader11(chunk_message_header) => {
                    will_return_full_chunk_message_header = chunk_message_header.into();
                    let read_size =
                        calc_read_size(will_return_full_chunk_message_header.message_length);
                    mut_reminder_message_length(
                        &mut will_return_full_chunk_message_header,
                        read_size,
                    );
                    read_size
                }
                ChunkMessageHeader::ChunkMessageHeader7(chunk_message_header) => {
                    will_return_full_chunk_message_header = FullChunkMessageHeader {
                        time_stamp: last_full_chunk_message_header.time_stamp
                            + chunk_message_header.time_stamp_delta,
                        msg_stream_id: last_full_chunk_message_header.msg_stream_id,
                        message_type: chunk_message_header.message_type.clone(),
                        message_length: chunk_message_header.message_length,
                        reminder_message_length: 0,
                    };
                    let read_size =
                        calc_read_size(will_return_full_chunk_message_header.message_length);
                    mut_reminder_message_length(
                        &mut will_return_full_chunk_message_header,
                        read_size,
                    );
                    read_size
                }
                ChunkMessageHeader::ChunkMessageHeader3(chunk_message_header) => {
                    will_return_full_chunk_message_header = FullChunkMessageHeader {
                        time_stamp: last_full_chunk_message_header.time_stamp
                            + chunk_message_header.time_stamp_delta,
                        message_length: last_full_chunk_message_header.message_length,
                        message_type: last_full_chunk_message_header.message_type.clone(),
                        msg_stream_id: last_full_chunk_message_header.msg_stream_id,
                        reminder_message_length: 0,
                    };
                    let read_size =
                        calc_read_size(will_return_full_chunk_message_header.message_length);
                    mut_reminder_message_length(
                        &mut will_return_full_chunk_message_header,
                        read_size,
                    );
                    read_size
                }
                ChunkMessageHeader::ChunkMessageHeader0(_) => {
                    let reminder_message_length =
                        last_full_chunk_message_header.reminder_message_length;

                    will_return_full_chunk_message_header = last_full_chunk_message_header.clone();
                    let mut result = 0;
                    if reminder_message_length > 0 {
                        result = calc_read_size(reminder_message_length);
                    } else {
                        result =
                            calc_read_size(will_return_full_chunk_message_header.message_length);
                        will_return_full_chunk_message_header.reminder_message_length =
                            will_return_full_chunk_message_header.message_length;
                    };
                    mut_reminder_message_length(&mut will_return_full_chunk_message_header, result);
                    result
                }
            }
        }
        // log::trace!(
        //     " will_read_message_length {:?} {:?} {:?}",
        //     will_read_message_length,
        //     will_return_full_chunk_message_header.message_length,
        //     will_return_full_chunk_message_header.reminder_message_length
        // );
        // 读取Data
        let bytes = async_read_num_byte(reader, will_read_message_length as usize)
            .await
            .unwrap();
        let chunk = Chunk {
            cs_id,
            chunk_header: chunk_message_header,
            message_data: bytes.to_vec(),
        };
        Ok((chunk, will_return_full_chunk_message_header))
    }
}

#[async_trait]
impl AsyncWriteByte for Chunk {
    async fn async_write_byte<Writer>(&self, writer: &mut Writer)
    where
        Writer: AW,
    {
        let mut bytes = BytesMut::new();
        /* todo  这里使用一个简单的实现，
        本来应该需要更具 最大chunk_size 切割message_body,处理对应的fmt组装成Chunk发送。
        这里先简单实现*/
        if self.cs_id < 64 {
            bytes.put_u8(self.cs_id.try_into().unwrap())
        } else if self.cs_id < 320 {
            let chunk_id = self.cs_id - 64;
            bytes.put_u8(0);
            bytes.put_u8(chunk_id as u8);
        } else {
            let chunk_id = self.cs_id - 64;
            bytes.put_u8(63);
            bytes.put_u16_le(chunk_id as u16);
        }

        match &self.chunk_header {
            ChunkMessageHeader::ChunkMessageHeader11(header) => {
                bytes.put_slice(&header.time_stamp.to_be_bytes()[1..4]);
                let payload_length = &(header.message_length as u32).to_be_bytes()[1..4];
                bytes.put_slice(&payload_length[..]);
                bytes.put_u8(header.message_type.clone().into());
                bytes.put_u32(header.message_stream_id);
                bytes.put_slice(&self.message_data[..]);
            }
            ChunkMessageHeader::ChunkMessageHeader7(header) => {
                bytes.put_slice(&header.time_stamp_delta.to_be_bytes()[1..4]);
                let payload_length = &(header.message_length as u32).to_be_bytes()[1..4];
                bytes.put_slice(&payload_length[..]);
                bytes.put_u8(header.message_type.clone().into());
                bytes.put_slice(&self.message_data[..]);
            }
            ChunkMessageHeader::ChunkMessageHeader3(header) => {
                bytes.put_slice(&header.time_stamp_delta.to_be_bytes()[1..4]);
                bytes.put_slice(&self.message_data[..]);
            }
            ChunkMessageHeader::ChunkMessageHeader0(header) => {
                bytes.put_slice(&self.message_data[..]);
            }
        }
        let _ = writer.write_all(&bytes[..]).await;
        writer.flush().await.unwrap();
    }
}

#[derive(Debug)]
pub enum ChunkMessageHeader {
    ChunkMessageHeader11(ChunkMessageHeader11),
    ChunkMessageHeader7(ChunkMessageHeader7),
    ChunkMessageHeader3(ChunkMessageHeader3),
    ChunkMessageHeader0(ChunkMessageHeader0),
}

#[derive(Debug, Clone)]
pub struct ChunkMessageHeader11 {
    // timestamp_delta:
    pub time_stamp: u32,
    pub message_length: u32,
    pub message_type: MessageType,
    pub message_stream_id: u32,
}

#[async_trait]
impl AsyncFrom for ChunkMessageHeader11 {
    async fn async_from<Reader>(reader: &mut Reader) -> Self
    where
        Reader: AR,
    {
        let mut bytes = async_read_num_byte(reader, 11).await.unwrap();
        let b32 = bytes.get_u32();
        let time_stamp = b32 >> 8;
        let reminder = (b32 << 24 >> 24) as u8;

        let r_message_length = bytes.get_u16().to_be_bytes();
        let mut rbytes =
            BytesMut::from_iter([0u8, reminder, r_message_length[0], r_message_length[1]].iter());

        let message_length = rbytes.get_u32();
        let message_type_id = bytes.get_u8();

        // TODO Message Type
        let message_type = message_type_id.into();
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

#[derive(Debug, Clone)]
pub struct ChunkMessageHeader7 {
    pub time_stamp_delta: u32,
    pub message_length: u32,
    pub message_type: MessageType,
}

#[async_trait]
impl AsyncFrom for ChunkMessageHeader7 {
    async fn async_from<Reader>(reader: &mut Reader) -> Self
    where
        Reader: AR,
    {
        let mut b4 = async_read_4_byte(reader).await.unwrap();
        let b4 = b4.get_u32();
        let time_stamp_delta = b4 >> 8;
        let message_length_header = b4 << 24 >> 24;
        let message_reminder = async_read_2_byte(reader).await.unwrap().get_u16();
        let message_length =
            (((message_length_header as u64) << 16) + message_reminder as u64) as u32;
        let message_type_id = async_read_1_byte(reader).await.unwrap().get_u8();
        let message_type = message_type_id.into();
        Self {
            time_stamp_delta,
            message_length,
            message_type: message_type,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChunkMessageHeader3 {
    pub time_stamp_delta: u32,
}

#[async_trait]
impl AsyncFrom for ChunkMessageHeader3 {
    async fn async_from<Reader>(reader: &mut Reader) -> Self
    where
        Reader: AR,
    {
        let mut b3 = async_read_3_byte(reader).await.unwrap();
        let b1 = b3.get_u8();
        let b2 = b3.get_u16();

        let time_stamp_delta = (b1 as u32).wrapping_shl(16) + b2 as u32;
        Self { time_stamp_delta }
    }
}

#[derive(Debug)]
pub struct ChunkMessageHeader0;

#[derive(Debug, Clone)]
pub struct FullChunkMessageHeader {
    pub time_stamp: u32,
    pub message_length: u32,
    pub message_type: MessageType,
    pub msg_stream_id: u32,
    pub reminder_message_length: u32,
}

impl Into<FullChunkMessageHeader> for &ChunkMessageHeader11 {
    fn into(self) -> FullChunkMessageHeader {
        FullChunkMessageHeader {
            time_stamp: self.time_stamp,
            message_length: self.message_length,
            reminder_message_length: 0,
            message_type: self.message_type.clone(),
            msg_stream_id: self.message_stream_id,
        }
    }
}
