use tokio::net::TcpListener;
use tokio::task::JoinHandle;

use super::protocol::accpect_rtmp;

pub fn start_server() -> JoinHandle<anyhow::Result<()>> {
    tokio::spawn(async {
        let listener = TcpListener::bind("127.0.0.1:1935").await?;
        loop {
            let (stream, _) = listener.accept().await?;
            accpect_rtmp(stream);
        }
    })
}
