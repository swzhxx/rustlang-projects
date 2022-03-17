use env_logger::Env;
use rtmp::start_server;
mod http_flv;
mod rtmp;
mod util;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::builder()
        .filter(Some("video_audio::http_flv"), log::LevelFilter::Trace)
        .init();
    let _flv_handle = http_flv::start_server();
    let join_handle = start_server();

    join_handle.await?.unwrap();
    Ok(())
}
