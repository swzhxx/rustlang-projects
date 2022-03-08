use async_trait::async_trait;

use super::{EventType, MessageType, SetPeerBandWidth, UserControlMessage, WindowAcknowledgement};
use crate::{
    rtmp::protocol::{message::Message, RtmpCtx},
    util::{read_all_amf_value, AsyncWriteByte, AW},
};
use amf::amf0;

trait FromAMF0Data {
    fn from_amf0_data(data: &[u8]) -> Option<Vec<amf::amf0::Value>>;
}

// 这里不再实现该amf3相关内容
trait FromAMF3Data {
    fn from_amf3_data(data: &[u8]) -> Option<Vec<amf::amf3::Value>>;
}

///  指令消息(20,17)
/// 指令消息在客户端和服务端之间传递AMF编码的指令，
/// 消息类型20代表AMF0编码，消息类型17代表AMF3编码。
/// 发送这些消息来完成连接、创建流、发布、播放、暂停等操作。
/// 像状态、结果这样的指令消息，用于通知发送方请求的指令状态。
/// 一条指令消息由指令名、事务ID和包含相关参数的指令对象。
/// 客户端或服务端还可以通过指令消息来实现远程过程调用(RPC)。
#[derive(Debug, Clone)]
pub struct CommandMessageAMF020;

impl FromAMF0Data for CommandMessageAMF020 {
    fn from_amf0_data(data: &[u8]) -> Option<Vec<amf::amf0::Value>> {
        read_all_amf_value(data)
    }
}

impl CommandMessageAMF020 {
    pub async fn excute<Writer>(
        data: &[u8],
        ctx: &mut RtmpCtx,
        writer: &mut Writer,
    ) -> anyhow::Result<()>
    where
        Writer: AW,
    {
        let command = CommandMessageAMF020::from_amf0_data(data);
        match command {
            Some(command) => {
                let values = command;
                for v in &values {
                    log::info!("[COMMAND MESSAGE {:?}]", v);
                }
                let command = values[0].try_as_str().unwrap();
                match command {
                    "connect" => {
                        Connect::excute_mut(ctx, writer).await;
                        Connect::response(ctx, writer).await;
                    }
                    "createStream" => {}
                    "publish" => {}
                    "play" => {}
                    _ => {}
                };

                Ok(())
            }
            None => Err(anyhow::anyhow!("[AMF0CommandMessage] expect AMF0 data"))?,
        }
    }

    pub async fn send<Writer>(command_type: Command, data: &Vec<amf0::Value>, writer: &mut Writer)
    where
        Writer: AW,
    {
        let cs_id = 2;
        let message_stream_id = 0;

        let mut message_body = vec![];
        let command_type: amf0::Value = (&command_type).into();

        for i in 0..data.len() {
            let data = &data[i];
            data.write_to(&mut message_body);
        }

        let message = Message::new(
            cs_id,
            MessageType::USER_CONTROL_MESSAGE(UserControlMessage),
            0,
            message_stream_id,
            message_body,
        );
        message.async_write_byte(writer).await;
    }
}

#[derive(Debug, Clone)]
pub struct CommandMessageAMF317;

pub enum Command {
    CONNECT(Connect),
    CREATE_STREAM(CreateStream),
    PUBLISH(Publish),
    PLAY(Play),
    UNKOWN,
}

impl Into<amf0::Value> for &Command {
    fn into(self) -> amf0::Value {
        match self {
            Command::CONNECT(_) => amf0::Value::String("connect".to_string()),
            Command::CREATE_STREAM(_) => amf0::Value::String("create_stream".to_string()),
            Command::PUBLISH(_) => amf0::Value::String("publish".to_string()),
            Command::PLAY(_) => amf0::Value::String("play".to_string()),
            Command::UNKOWN => amf0::Value::String("unknown".to_string()),
        }
    }
}

#[async_trait]
pub trait CommandExcute {
    async fn excute(ctx: &RtmpCtx) {}
}

#[async_trait]
pub trait CommandExcuteMut {
    async fn excute_mut<Writer>(ctx: &mut RtmpCtx, writer: &mut Writer)
    where
        Writer: AW;
}

#[async_trait]
pub trait CommandResponse {
    async fn response<Writer>(ctx: &mut RtmpCtx, writer: &mut Writer)
    where
        Writer: AW;
}

#[derive(Debug, Clone)]
pub struct Connect;

#[async_trait]
impl CommandExcuteMut for Connect {
    async fn excute_mut<Writer>(ctx: &mut RtmpCtx, writer: &mut Writer)
    where
        Writer: AW,
    {
        WindowAcknowledgement::send(4096, ctx, writer).await;
        SetPeerBandWidth::send(4096, super::LimitType::Hard, ctx, writer).await;
        UserControlMessage::send(EventType::STREAM_BEGIN, writer);
    }
}

#[async_trait]
impl CommandResponse for Connect {
    async fn response<Writer>(ctx: &mut RtmpCtx, writer: &mut Writer)
    where
        Writer: AW,
    {
        let mut amf_datas = vec![];
        amf_datas.push(amf0::Value::String("_result".to_string()));
        amf_datas.push(amf0::Value::Number(1.0));
        amf_datas.push(amf0::Value::Object {
            class_name: None,
            entries: vec![
                amf::Pair {
                    key: "fmsVer".to_owned(),
                    value: amf::amf0::Value::String("FMS/3,0,1,123".to_owned()),
                },
                amf::Pair {
                    key: "capabilities".to_owned(),
                    value: amf::amf0::Value::Number(31.0),
                },
            ],
        });
        amf_datas.push(amf::amf0::Value::Object {
            class_name: None,
            entries: vec![
                amf::Pair {
                    key: "level".to_owned(),
                    value: amf::amf0::Value::String("status".to_owned()),
                },
                amf::Pair {
                    key: "code".to_owned(),
                    value: amf::amf0::Value::String("NetConnection.Connect.Success".to_owned()),
                },
                amf::Pair {
                    key: "description".to_owned(),
                    value: amf::amf0::Value::String("Connection succeeded.".to_owned()),
                },
                amf::Pair {
                    key: "objectEncoding".to_owned(),
                    value: amf::amf0::Value::Number(0.0),
                },
            ],
        });
        CommandMessageAMF020::send(Command::CONNECT(Connect), &amf_datas, writer).await;
    }
}

#[derive(Debug, Clone)]
pub struct CreateStream;

#[derive(Debug, Clone)]
pub struct Publish;

pub struct Play;
