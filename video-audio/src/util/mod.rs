pub mod async_from;
mod effect_bytes;

use std::io::Write;

pub use async_from::*;
use async_trait::async_trait;
pub use effect_bytes::*;
use tokio::io::AsyncWrite;

pub trait AsBytes {
    fn as_bytes<'a, 'b>(&'a self) -> &'b [u8];
}

pub trait ToBytes<const N: usize> {
    fn to_bytes(&self) -> [u8; N];
}

#[async_trait]
pub trait AsyncWriteByte<T> {
    async fn async_write_byte(&self, _: T);
}
