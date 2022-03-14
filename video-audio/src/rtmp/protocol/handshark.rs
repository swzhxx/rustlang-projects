use std::{collections::HashMap, io::BufWriter};

use async_trait::async_trait;
use bytes::{Buf, BufMut, BytesMut};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufStream},
    net::{TcpSocket, TcpStream},
};

use crate::util::*;

#[derive(Debug, Clone)]
pub struct HandShark0 {
    pub version: u8,
}

impl Default for HandShark0 {
    fn default() -> Self {
        Self { version: 3 }
    }
}

#[async_trait]
impl AsyncFrom for HandShark0 {
    async fn async_from<Reader>(reader: &mut Reader) -> Self
    where
        Reader: AR,
    {
        let mut bytes = async_read_1_byte(reader).await.unwrap();
        let version = bytes.get_u8();
        return HandShark0 { version };
    }
}

#[async_trait]
impl AsyncWriteByte for HandShark0 {
    async fn async_write_byte<Writer>(&self, writer: &mut Writer)
    where
        Writer: AW,
    {
        writer.write_u8(self.version).await;
        writer.flush().await;
    }
}

#[derive(Debug)]
pub struct HandShark1 {
    pub time: u32,
    pub zero: u32,
    // 任意值，但是没有使用加密的安全的随机值和动态值
    pub random_data: Vec<u8>,
}

impl Default for HandShark1 {
    fn default() -> Self {
        Self {
            time: Default::default(),
            zero: Default::default(),
            random_data: vec![0; 1528],
        }
    }
}

#[async_trait]
impl AsyncFrom for HandShark1 {
    async fn async_from<Reader>(reader: &mut Reader) -> Self
    where
        Reader: AR,
    {
        let mut bytes = async_read_num_byte(reader, 1536).await.unwrap();
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
impl AsyncWriteByte for HandShark1 {
    async fn async_write_byte<Writer>(&self, writer: &mut Writer)
    where
        Writer: AW,
    {
        writer.write_u32(self.time).await;
        writer.write_u32(self.zero).await;
        writer.write_all(&self.random_data).await;
        writer.flush().await;
    }
}

#[derive(Debug)]
pub struct HandShark2 {
    pub time1: u32,
    pub time2: u32,
    pub random_echo: Vec<u8>,
}

#[async_trait]
impl AsyncFrom for HandShark2 {
    async fn async_from<Reader>(reader: &mut Reader) -> Self
    where
        Reader: AR,
    {
        let mut bytes = async_read_num_byte(reader, 1536).await.unwrap();
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

#[async_trait]
impl AsyncWriteByte for HandShark2 {
    async fn async_write_byte<Writer>(&self, writer: &mut Writer)
    where
        Writer: AW,
    {
        writer.write_u32(self.time1).await;
        writer.write_u32(self.time2).await;
        writer.write_all(&self.random_echo).await;
        writer.flush().await;
    }
}

#[derive(Debug)]
pub enum HandSharkState {
    C0(HandShark0),
    S0(HandShark0),

    C1(HandShark1),
    S1(HandShark0),

    C2(HandShark2),
    S2(HandShark2),
}
