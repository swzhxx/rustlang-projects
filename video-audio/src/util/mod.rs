pub mod async_from;

use std::io::Write;

pub use async_from::*;

pub trait AsBytes {
    fn as_bytes<'a, 'b>(&'a self) -> &'b [u8];
}

pub trait ToBytes<const N: usize> {
    fn to_bytes(&self) -> [u8; N];
}

pub trait WriteByte<T>
where
    T: Write,
{
    fn write_byte(&self, _: T);
}
