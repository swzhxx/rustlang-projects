use bytes::{Buf, BufMut, BytesMut};

use crate::rtmp::protocol::message::Message;

pub const FLV_HEADER_WITH_TAG0: [u8; 13] = [
    0x46, 0x4c, 0x56, // signature
    0x01, // version
    0x05, // audio and video flag
    0x00, 0x00, 0x00, 0x09, // header length
    0x00, 0x00, 0x00, 0x00, // tag0 length
];

pub struct FlvTag {
    raw_data: Vec<u8>,
}

impl FlvTag {
    pub fn tag_type(&self) -> u8 {
        self.raw_data[0]
    }

    pub fn data_size(&self) -> u32 {
        let mut bytes = BytesMut::new();
        bytes.put_slice(&[0, self.raw_data[1], self.raw_data[2], self.raw_data[3]][..]);
        let size = bytes.get_u32();
        size
    }

    pub fn timestamp(&self) -> u32 {
        let mut bytes = BytesMut::new();
        bytes.put_slice(&[0, self.raw_data[4], self.raw_data[5], self.raw_data[6]][..]);
        let mut timestamp = bytes.get_u32();
        if timestamp == 1677215 {
            let extend_timestamp = self.raw_data[7] as u32;
            timestamp = extend_timestamp << 24 | timestamp
        }
        timestamp
    }

    pub fn body(&self) -> &[u8] {
        &self.raw_data[11..]
    }
}

impl TryFrom<Message> for FlvTag {
    type Error = anyhow::Error;

    fn try_from(mut message: Message) -> Result<Self, Self::Error> {
        let mut raw_data = vec![];
        match message.message_type {
            crate::rtmp::protocol::message::MessageType::AUDIO_MESSAGE(_) => {
                raw_data.push(0x08);
            }
            crate::rtmp::protocol::message::MessageType::VIDEO_MESSAGE(_) => {
                raw_data.push(0x09);
            }
            _ => {}
        };
        raw_data.extend_from_slice(&(message.message_body.len() as u32).to_be_bytes()[1..4]);
        raw_data.extend_from_slice(&(message.time_stamp & 0xFFFFFF).to_be_bytes()[1..4]);
        raw_data.push((message.time_stamp >> 24) as u8);
        raw_data.extend_from_slice(&(0u32.to_be_bytes()[1..4]));
        raw_data.append(&mut message.message_body);
        Ok(FlvTag { raw_data })
    }
}

impl AsRef<[u8]> for FlvTag {
    fn as_ref(&self) -> &[u8] {
        self.raw_data.as_ref()
    }
}
