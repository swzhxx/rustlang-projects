use tokio::net::TcpListener;
use tokio::task::JoinHandle;

use super::protocol::accpect_rtmp;

pub fn start_server() -> JoinHandle<anyhow::Result<()>> {
    tokio::spawn(async {
        let str = "0.0.0.0:1935";
        let listener = TcpListener::bind(str).await?;
        log::info!("[START RTMP SERVER {}]", str);
        loop {
            let (stream, _) = listener.accept().await?;
            log::info!("[PEER ADDR {:?} CONNECT]", stream.peer_addr());
            tokio::spawn(async {
                let _ = accpect_rtmp(stream).await;
            });
        }
    })
}
