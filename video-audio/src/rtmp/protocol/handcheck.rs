use std::io::BufWriter;

use async_trait::async_trait;
use bytes::{Buf, BufMut, BytesMut};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufStream},
    net::{TcpSocket, TcpStream},
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
        let mut bytes = async_read_1_byte(stream).await;
        let version = bytes.get_u8();
        return HandCheck0 { version };
    }
}

#[async_trait]
impl AsyncWriteByte<&'static mut TcpStream> for HandCheck0 {
    async fn async_write_byte(&self, stream: &'static mut TcpStream) {
        stream.write_u8(self.version);
    }
}

#[derive(Debug)]
pub struct HandCheck1 {
    pub time: u32,
    pub zero: u32,
    // 任意值，但是没有使用加密的安全的随机值和动态值
    pub random_data: Vec<u8>,
}

impl Default for HandCheck1 {
    fn default() -> Self {
        Self {
            time: Default::default(),
            zero: Default::default(),
            random_data: vec![12; 1528],
        }
    }
}

#[async_trait]
impl AsyncFrom<&'static mut TcpStream> for HandCheck1 {
    async fn async_from(stream: &'static mut TcpStream) -> Self {
        let mut bytes = async_read_num_byte(stream, 1536).await;
        let time = bytes.get_u32();
        let zero = bytes.get_u32();
        let random_data = bytes.get(0..1528).unwrap().to_vec();
        bytes.advance(1528);
        Self {
            time,
            zero,
            random_data: random_data,
        }
    }
}

#[async_trait]
impl AsyncWriteByte<&'static mut TcpStream> for HandCheck1 {
    async fn async_write_byte(&self, stream: &'static mut TcpStream) {
        stream.write_u32(self.time);
        stream.write_u32(self.zero);
        stream.write_all(&self.random_data).await;
    }
}

#[derive(Debug)]
pub struct HandCheck2 {
    pub time1: u32,
    pub time2: u32,
    pub random_echo: Vec<u8>,
}

#[async_trait]
impl AsyncFrom<&'static mut TcpStream> for HandCheck2 {
    async fn async_from(stream: &'static mut TcpStream) -> Self {
        let mut bytes = async_read_num_byte(stream, 1536).await;
        let time1 = bytes.get_u32();
        let time2 = bytes.get_u32();
        let random_echo = bytes.get(0..1528).unwrap().to_vec();
        bytes.advance(1528);
        Self {
            time1,
            time2,
            random_echo: random_echo,
        }
    }
}

#[derive(Debug)]
pub enum HandCheckState {
    c0(HandCheck0),
    s0(HandCheck0),
    s1(HandCheck0),
    c1(HandCheck1),
    c2(HandCheck2),
    s2(HandCheck2),
}
