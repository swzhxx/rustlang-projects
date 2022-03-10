use amf::amf0;
use bytes::{Buf, BytesMut};

use crate::{
    rtmp::protocol::message::{self, Message, MessageType},
    util::{AsyncWriteByte, AW},
};

/// 用户控制消息
#[derive(Debug, Clone)]
pub struct UserControlMessage;

impl UserControlMessage {
    pub async fn excute() {
        todo!()
    }
    pub async fn send<Writer>(event_type: EventType, writer: &mut Writer)
    where
        Writer: AW,
    {
        let cs_id = 2;
        let message_stream_id = 0;

        let message_body = (event_type as u32).to_be_bytes().to_vec();
        let message = Message::new(
            cs_id,
            MessageType::USER_CONTROL_MESSAGE(UserControlMessage),
            0,
            0,
            message_body,
        );
        
        message.async_write_byte(writer).await;
    }
}

pub enum EventType {
    STREAM_BEGIN = 0,
    STREAM_EOF = 1,
    STREAM_DRY = 2,
    SET_BUFFER_LENGTH = 3,
    STREAM_IS_RECORD = 4,
    PING_REQUEST = 5,
    PING_RESPONSE = 6,
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
                    0 => Ok(EventType::STREAM_BEGIN),
                    1 => Ok(EventType::STREAM_EOF),
                    2 => Ok(EventType::STREAM_DRY),
                    3 => Ok(EventType::SET_BUFFER_LENGTH),
                    4 => Ok(EventType::STREAM_IS_RECORD),
                    5 => Ok(EventType::PING_REQUEST),
                    6 => Ok(EventType::PING_RESPONSE),
                    _ => return Err("message to eventype error".to_string()),
                }
            }
            _ => return Err("message to eventype error".to_string()),
        }
    }
}
