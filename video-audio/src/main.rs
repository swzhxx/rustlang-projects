use env_logger::Env;
use rtmp::start_server;

mod rtmp;
mod util;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::builder()
        .filter(Some("video_audio"), log::LevelFilter::Trace)
        .init();

    let join_handle = start_server();
    join_handle.await?.unwrap();
    Ok(())
}
