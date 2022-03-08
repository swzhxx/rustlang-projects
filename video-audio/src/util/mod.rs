mod effect_bytes;

use std::io::Write;

use amf::amf0;
use amf::amf0::Value;
use async_trait::async_trait;
pub use effect_bytes::*;
use rand::Rng;
use tokio::io::{AsyncRead, AsyncWrite};

pub trait AsBytes {
    fn as_bytes<'a, 'b>(&'a self) -> &'b [u8];
}

pub trait ToBytes<const N: usize> {
    fn to_bytes(&self) -> [u8; N];
}

pub trait AW: AsyncWrite + Sync + Send + Unpin {}
impl<T> AW for T where T: AsyncWrite + Sync + Send + Unpin {}

#[async_trait]
pub trait AsyncWriteByte {
    async fn async_write_byte<T>(&self, _: &mut T)
    where
        T: AW;
}

pub trait AR: AsyncRead + Sync + Send + Unpin {}
impl<T> AR for T where T: AsyncRead + Sync + Send + Unpin {}

#[async_trait]
pub trait AsyncFrom {
    async fn async_from<T>(_: &mut T) -> Self
    where
        T: AR;
}

#[async_trait]
pub trait TryAsyncFrom {
    async fn try_async_from<T>(_: &mut T) -> anyhow::Result<Self>
    where
        T: AR,
        Self: Sized;
}

/// 生成随机字节数组
pub fn gen_random_bytes(len: u32) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut vec = Vec::new();
    for _ in 0..len {
        vec.push(rng.gen());
    }
    vec
}

/// 计算一个AMF值的字节长度
pub fn calc_amf_byte_len(v: &amf0::Value) -> usize {
    match v {
        Value::Number(_) => 9,
        Value::Boolean(_) => 2,
        Value::String(s) => (s.len() + 3),
        Value::Object { entries, .. } => {
            // marker and tail
            let mut len = 4;
            for en in entries {
                len += en.key.len() + 2;
                len += calc_amf_byte_len(&en.value);
            }
            len
        }
        Value::Null => 1,
        Value::Undefined => 1,
        Value::EcmaArray { entries } => {
            // marker and tail
            let mut len = 8;
            for en in entries {
                len += en.key.len() + 2;
                len += calc_amf_byte_len(&en.value);
            }
            len
        }
        Value::Array { entries: _ } => unimplemented!(),
        Value::Date { unix_time: _, time_zone } => unimplemented!(),
        Value::XmlDocument(_) => unimplemented!(),
        Value::AvmPlus(_) => unimplemented!(),
    }
}

/// 从字节数组中读取全部的AMF值
pub fn read_all_amf_value(bytes: &[u8]) -> Option<Vec<Value>> {
    let mut read_num = 0;
    let mut list = Vec::new();

    loop {
        if let Ok(v) = amf::amf0::Value::read_from(&mut &bytes[read_num..]) {
            let len = calc_amf_byte_len(&v);
            read_num += len;
            list.push(v);

            if read_num >= bytes.len() {
                break;
            }
        } else {
            return None;
        }
    }
    Some(list)
}
