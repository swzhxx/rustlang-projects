mod messages;

use std::collections::HashMap;

use async_trait::async_trait;
pub use messages::*;
use tokio::io::AsyncWriteExt;

use crate::util::{AsyncWriteByte, AW};

use super::chunk::{Chunk, FullChunkMessageHeader};

#[derive(Debug, Clone)]
pub struct Message {
    pub message_type: MessageType,
    payload_length: u32,
    pub time_stamp: u32,
    pub message_stream_id: u32,
    pub message_body: Vec<u8>,
}

impl Message {
    fn from_chunks(mut chunks: Vec<Chunk>, full_chunk_descr: &FullChunkMessageHeader) -> Self {
        let message_type = full_chunk_descr.message_type.clone();
        let message_stream_id = full_chunk_descr.msg_stream_id;
        let time_stamp = full_chunk_descr.time_stamp;
        let body = chunks.iter_mut().fold(vec![], |mut body, chunk| {
            body.append(&mut chunk.message_data);
            return body;
        });
        let payload_length = body.len() as u32;
        Self {
            message_type,
            payload_length,
            time_stamp,
            message_stream_id,
            message_body: body,
        }
    }
}

impl Message {
    pub fn get_payload_length(&self) -> u32 {
        self.payload_length
    }
}

#[async_trait]
impl AsyncWriteByte for Message {
    async fn async_write_byte<Writer>(&self, writer: &mut Writer)
    where
        Writer: AW,
    {
        let message_type: u8 = self.message_type.clone().into();
        let payload_length = &self.message_body.len().to_be_bytes()[1..3];

        let message_stream_id = &self.message_stream_id.to_be_bytes()[1..3];

        writer.write_u8(message_type).await;
        writer.write_all(payload_length).await;
        writer.write_u32(self.time_stamp).await;
        writer.write_all(message_stream_id).await;
        // 写入Payload
        writer.write_all(&self.message_body).await;
    }
}

#[derive(Debug)]
pub struct MessageFactor {
    chunk_hash_map: HashMap<u32, Vec<Chunk>>,
}

impl MessageFactor {
    fn add_chunk(
        &mut self,
        chunk: Chunk,
        full_chunk_descr: &FullChunkMessageHeader,
    ) -> Option<Message> {
        let is_enough = {
            let data = &chunk.message_data;
            let message_length = full_chunk_descr.message_length as usize;
            message_length == data.len()
        };
        if is_enough {
            // todo 转化为Message
            return Some(Message::from_chunks(vec![chunk], full_chunk_descr));
        } else {
            let cs_id = chunk.cs_id;
            let chunk_hash_map = self.chunk_hash_map.get_mut(&cs_id);
            match chunk_hash_map {
                Some(chunks) => {
                    let current_len = chunks.iter().fold(chunk.message_data.len(), |acc, chunk| {
                        return acc + chunk.message_data.len();
                    });
                    if current_len == full_chunk_descr.message_length as usize {
                        // todo 转化为message
                        if let Some(chunks) = self.chunk_hash_map.remove(&cs_id) {
                            let message = Message::from_chunks(chunks, full_chunk_descr);
                            return Some(message);
                        }
                    } else {
                        chunks.push(chunk);
                    }
                }
                None => {
                    let chunks = vec![chunk];
                    self.chunk_hash_map.insert(cs_id, chunks);
                }
            }
        }
        None
    }
}
