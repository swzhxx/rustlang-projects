mod messages;

use std::collections::HashMap;

use async_trait::async_trait;
use bytes::{BufMut, BytesMut};
pub use messages::*;
use tokio::io::AsyncWriteExt;

use crate::{
    rtmp::protocol::{
        chunk::{ChunkMessageHeader, ChunkMessageHeader11},
        message,
    },
    util::{AsyncWriteByte, AR, AW},
};

use super::{
    chunk::{Chunk, FullChunkMessageHeader},
    RtmpCtx,
};

#[derive(Debug, Clone)]
pub struct Message {
    pub message_type: MessageType,
    payload_length: u32,
    pub time_stamp: u32,
    pub message_stream_id: u32,
    pub message_body: Vec<u8>,
    pub chunk_id: u32,
}

impl Message {
    fn new(
        chunk_id: u32,
        message_type: MessageType,
        time_stamp: u32,
        message_stream_id: u32,
        message_body: Vec<u8>,
    ) -> Self {
        let payload_length = message_body.len() as u32;
        Self {
            chunk_id,
            payload_length,
            message_type,
            time_stamp,
            message_stream_id,
            message_body,
        }
    }
    fn from_chunks(mut chunks: Vec<Chunk>, full_chunk_descr: &FullChunkMessageHeader) -> Self {
        log::trace!("[MESSAGE FROM CHUNKS] -> {:?}", chunks.len());
        let message_type = full_chunk_descr.message_type.clone();
        let message_stream_id = full_chunk_descr.msg_stream_id;
        let time_stamp = full_chunk_descr.time_stamp;
        let chunk_id = match chunks.get(0) {
            Some(chunk) => chunk.cs_id,
            None => {
                log::error!("[MESSAGE FROM CHUNKS CAN NOT FIND CHUNK]");
                0
            }
        };
        let body = chunks.iter_mut().fold(vec![], |mut body, chunk| {
            body.append(&mut chunk.message_data);
            return body;
        });
        let payload_length = body.len() as u32;
        Self {
            chunk_id,
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

    pub async fn dispatch<'a, T>(&self, ctx: &'a mut RtmpCtx, stream: &mut T)
    where
        T: AR + AW,
    {
        let chunk_data = &self.message_body[..];
        match &self.message_type {
            MessageType::UNKOWN => {
                todo!()
            }
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
                SetPeerBandWidth::excute(chunk_data, ctx, stream).await;
            }
            MessageType::COMMAND_MESSAGE_AMF0_20(_) => {
                CommandMessageAMF020::excute(chunk_data, ctx, stream).await;
            }
            MessageType::DATA_MESSAGE_18(_) => {
                DataMessage18::excute(chunk_data, ctx, stream).await;
            }
            MessageType::AUDIO_MESSAGE(_) => {
                AudioMessage::excute(chunk_data, ctx, stream, self).await;
            }
            MessageType::VIDEO_MESSAGE(_) => {
                VideoMessage::excute(chunk_data, ctx, stream, self).await;
            }
            MessageType::USER_CONTROL_MESSAGE(_) => {
                UserControlMessage::excute(chunk_data, ctx, stream, self).await;
            }
            _ => {
                log::error!("UNKNOWN MESSAGE EXCUTE")
            }
        }
    }

    pub fn split_chunks(&self, ctx: &RtmpCtx) -> Vec<Chunk> {
        let chunk_id = self.chunk_id;
        let mut message_body = &self.message_body[0..];
        let chunk_size = ctx.chunk_size as usize;
        let mut chunks = vec![];
        loop {
            let mut chunk = Chunk::default();
            if chunks.len() == 0 {
                chunk.cs_id = chunk_id;
                let chunk_message_header = ChunkMessageHeader11 {
                    time_stamp: self.time_stamp,
                    message_length: self.message_body.len() as u32,
                    message_type: self.message_type.clone(),
                    message_stream_id: self.message_stream_id,
                };
                chunk.chunk_header = ChunkMessageHeader::ChunkMessageHeader11(chunk_message_header);
                let size = message_body.len().min(chunk_size);
                log::error!("size {:?}  {:?}", size, chunk_size);
                chunk.message_data = (&message_body[0..size]).to_vec();
                message_body = &message_body[size..];
            } else {
                chunk.cs_id = chunk_id;
                let size = message_body.len().min(chunk_size);
                chunk.message_data = (&message_body[0..size]).to_vec();

                message_body = &message_body[size..];
            }
            chunks.push(chunk);
            if message_body.len() == 0 {
                break;
            }
        }
        chunks
    }
}

impl Message {
    async fn async_write_byte<Writer>(&self, ctx: &RtmpCtx, writer: &mut Writer)
    where
        Writer: AW,
    {
        log::trace!("[MESSAGE SEND] -> {:?}", self.message_type);
        let chunks = self.split_chunks(ctx);
        let iter = chunks.iter();
        for chunk in iter {
            chunk.async_write_byte(writer).await;
        }
    }
}

#[derive(Debug)]
pub struct MessageFactor {
    chunk_hash_map: HashMap<u32, Vec<Chunk>>,
}

impl MessageFactor {
    pub fn new() -> Self {
        Self {
            chunk_hash_map: HashMap::new(),
        }
    }
    pub fn add_chunk(
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
            log::trace!("[MESSAGE is_enough] -> {}", full_chunk_descr.message_length);
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
                        chunks.push(chunk);
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
