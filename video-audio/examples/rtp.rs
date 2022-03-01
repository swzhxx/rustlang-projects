use std::io::BufReader;

use anyhow::*;
use bytes::{BufMut, BytesMut};
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, UdpSocket},
};

struct RTPHeader {
    version: u8,
    padding: bool,
    extend: bool,
    marker: bool,
    payload: u8,
    sn: u16,
    time_stamp: u32,
    ssrc: u32,
    csrc_len: u32,
}

fn main() {}
