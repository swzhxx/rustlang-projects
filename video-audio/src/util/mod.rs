mod effect_bytes;

use std::io::Write;

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
