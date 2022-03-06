use std::pin::Pin;

use tokio::io::AsyncRead;

pub struct AsyncReaderEffect<'a, Read> {
    reader: &'a mut Read,
    read_bytes_num: usize,
}

impl<'a, Read> AsyncReaderEffect<'a, Read> {
    pub fn new(reader: &'a mut Read) -> Self {
        Self {
            reader,
            read_bytes_num: 0,
        }
    }

    pub fn get_readed_bytes_num(&self) -> usize {
        self.read_bytes_num
    }
}

impl<'a, Read> AsyncRead for AsyncReaderEffect<'a, Read>
where
    Read: AsyncRead + Unpin,
{
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        let are = self.get_mut();
        let before_filled = buf.filled().len();
        let result = AsyncRead::poll_read(Pin::new(are.reader), cx, buf);
        let filled = buf.filled().len();
        are.read_bytes_num += filled - before_filled;
        return result;
    }
}
