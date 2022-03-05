use tokio::io::AsyncRead;

pub struct ReadEffect<'a, Read> {
    stream: &'a mut Read,
    read_bytes_num: usize,
}

impl<'a, Read> ReadEffect<'a, Read> {
    fn new(stream: &'a mut Read) -> Self {
        Self {
            stream,
            read_bytes_num: 0,
        }
    }
}

impl<'a, Read> AsyncRead for ReadEffect<'a, Read> {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        todo!()
    }
}
