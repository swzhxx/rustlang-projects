use bytes::{BufMut, BytesMut};
use tokio::io::{AsyncRead, AsyncReadExt};

pub async fn async_read_num_byte<T>(reader: &mut T, size: usize) -> BytesMut
where
    T: AsyncRead + Unpin + Sync + Send,
{
    let mut bytes = BytesMut::with_capacity(size);
    bytes.put_slice(&vec![0; size]);
    let _ = AsyncReadExt::read_exact(reader, &mut bytes).await;
    bytes
}

pub async fn async_read_1_byte<T>(reader: &mut T) -> BytesMut
where
    T: AsyncRead + Unpin + Sync + Send,
{
    async_read_num_byte(reader, 1).await
}

pub async fn async_read_2_byte<T>(reader: &mut T) -> BytesMut
where
    T: AsyncRead + Unpin + Sync + Send,
{
    async_read_num_byte(reader, 2).await
}

pub async fn async_read_3_byte<T>(reader: &mut T) -> BytesMut
where
    T: AsyncRead + Unpin + Sync + Send,
{
    async_read_num_byte(reader, 3).await
}

pub async fn async_read_4_byte<T>(reader: &mut T) -> BytesMut
where
    T: AsyncRead + Unpin + Sync + Send,
{
    async_read_num_byte(reader, 4).await
}
