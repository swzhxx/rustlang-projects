use std::any;

use chrono::{Local, TimeZone};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    task::JoinHandle,
};

use crate::{http_flv::flv::FlvTag, rtmp::protocol::eventbus_map, util::EventBus};

pub mod flv;

pub fn start_server() -> JoinHandle<anyhow::Result<()>> {
    let addr = "0.0.0.0:8080";
    tokio::spawn(async move {
        let listener = TcpListener::bind(addr).await?;
        let addr = format!("http://{}", listener.local_addr()?);
        log::info!("HTTP-FLV Server is listening to {}", addr);
        loop {
            let (stream, _) = listener.accept().await?;
            tokio::spawn(async move {
                let _ = handle_accept(stream).await;
            });
        }
    })
}

async fn handle_accept(mut stream: TcpStream) -> anyhow::Result<()> {
    log::trace!("{:?} in", stream.peer_addr());
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await?;
    let req = String::from_utf8_lossy(&buffer[..]);
    let stream_name = get_path(req.as_ref())
        .map(|x| x.trim_start_matches("/"))
        .unwrap_or_default();
    log::trace!("[HTTP FLV] -> stream_name {}", stream_name);
    let event_bus = match eventbus_map().get(stream_name.clone()) {
        Some(event_bus) => event_bus,
        None => {
            eventbus_map().insert(
                stream_name.to_string(),
                EventBus::new_with_label(stream_name.to_string()),
            );
            eventbus_map().get(stream_name.clone()).unwrap()
        }
    };
    let mut receiver = event_bus.register_receiver();
    let header = "HTTP/1.1 200 OK\r\n\
    Server: That's\r\n\
    Content-Type: video/x-flv\r\n\
    Connection: close\r\n\
    Transfer-Encoding: chunked\r\n\
    Cache-Control: no-cache\r\n\
    Access-Control-Allow-Origin: *\r\n\
    \r\n\
    ";

    stream.write_all(header.as_bytes()).await?;
    stream.flush().await?;
    let ctx_begin_stamp = Local::now().timestamp_millis();
    while let Ok(mut msg) = receiver.recv().await {
        match msg.message_type {
            crate::rtmp::protocol::message::MessageType::AUDIO_MESSAGE(_)
            | crate::rtmp::protocol::message::MessageType::VIDEO_MESSAGE(_) => {
                msg.time_stamp = (Local::now().timestamp_millis() - ctx_begin_stamp) as u32;
                let flv_tag = FlvTag::try_from(msg)?;
                write_chunk(&mut stream, flv_tag.as_ref()).await?;
                write_chunk(&mut stream, &(flv_tag.as_ref().len() as u32).to_be_bytes()).await?;
            }
            _ => {}
        }
    }
    write_chunk(&mut stream, b"").await?;
    Ok(())
}

fn get_path(req: &str) -> Option<&str> {
    let first_line = req.lines().next().unwrap_or_default();
    if first_line.starts_with("GET") {
        return first_line.split_whitespace().skip(1).next();
    }
    None
}

async fn write_chunk(stream: &mut TcpStream, bytes: &[u8]) -> anyhow::Result<()> {
    stream
        .write_all(format!("{:X}\r\n", bytes.len()).as_bytes())
        .await?;
    stream.write_all(bytes).await?;
    stream.write_all(b"\r\n").await?;
    stream.flush().await?;
    Ok(())
}
