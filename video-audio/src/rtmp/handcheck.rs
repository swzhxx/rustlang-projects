use std::io::BufWriter;

use async_trait::async_trait;
use bytes::{Buf, BufMut, BytesMut};
use tokio::{
    io::{AsyncReadExt, BufStream},
    net::TcpStream,
};

use crate::util::*;

#[derive(Debug, Clone)]
pub struct HandCheck0 {
    pub version: u8,
}

impl Default for HandCheck0 {
    fn default() -> Self {
        Self { version: 3 }
    }
}

#[async_trait]
impl AsyncFrom<&'static mut TcpStream> for HandCheck0 {
    async fn async_from(stream: &'static mut TcpStream) -> Self {
        let mut bytes = BytesMut::with_capacity(1);
        bytes.put_u8(0);
        stream.read_exact(&mut bytes).await;
        // stream.read
        let version = bytes.get_u8();
        return HandCheck0 { version };
    }
}

// impl AsBytes for HandCheck0 {
//     fn as_bytes<'a, 'b>(&'a self) -> &'b [u8] {
//         self.version.byte
//     }
// }

// impl ToBytes<1> for HandCheck0 {
//     fn to_bytes(&self) -> [u8; 1] {
//         self.version.to_be_bytes()
//     }
// }

#[derive(Debug)]
struct HandCheck1 {
    time: u32,
    zero: u32,
    random_data: Vec<u8>,
}

#[async_trait]
impl AsyncFrom<&'static mut TcpStream> for HandCheck1 {
    async fn async_from(stream: &'static mut TcpStream) -> Self {
        let mut bytes = BytesMut::with_capacity(1536);
        bytes.put_slice(&[0; 1536]);
        let _size = stream.read_exact(&mut bytes).await;
        let time = bytes.get_u32();
        let zero = bytes.get_u32();
        let random_data = bytes.get(0..1528).unwrap();
        Self {
            time,
            zero,
            random_data: random_data.to_vec(),
        }
    }
}
