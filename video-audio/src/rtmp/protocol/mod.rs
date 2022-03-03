use anyhow::Result;
use tokio::{net::TcpStream, stream};

use crate::{
    rtmp::protocol::handshark::{HandShark1, HandShark2},
    util::{gen_random_bytes, AsyncFrom, AsyncWriteByte},
};

use self::handshark::{HandShark0, HandSharkState};
mod chunk;
pub mod handshark;

#[derive(Debug)]
struct RtmpCtx {
    receive_handshark_state: Option<HandSharkState>,
    send_handshark_state: Option<HandShark1>,
    ctx_begig_timestamp: i64,
}

impl Default for RtmpCtx {
    fn default() -> Self {
        Self {
            receive_handshark_state: Default::default(),
            send_handshark_state: Default::default(),
            ctx_begig_timestamp: chrono::Local::now().timestamp_millis(),
        }
    }
}

impl RtmpCtx {
    async fn handle_hand_check(&mut self, stream: &mut TcpStream) -> anyhow::Result<()> {
        loop {
            match &self.receive_handshark_state {
                Some(receive_handshark_state) => match receive_handshark_state {
                    HandSharkState::C0(_) => {
                        let s0 = HandShark0::default();
                        s0.async_write_byte(stream).await;
                        log::trace!("[SEND]->S0");
                        let mut s1 = HandShark1::default();
                        s1.time = (chrono::Local::now().timestamp_millis()
                            - self.ctx_begig_timestamp) as u32;
                        s1.random_data = gen_random_bytes(1528);
                        s1.async_write_byte(stream).await;
                        log::trace!("[SEND] -> S1");
                        self.send_handshark_state = Some(s1);

                        let c1 = HandShark1::async_from(stream).await;
                        log::trace!("[RECEIVE] -> C1 time:{:?}", c1.time);
                        self.receive_handshark_state = Some(HandSharkState::C1(c1));
                    }
                    HandSharkState::C1(c1) => {
                        let s2 = HandShark2 {
                            time1: c1.time,
                            time2: 0,
                            random_echo: c1.random_data.clone(),
                        };

                        s2.async_write_byte(stream).await;
                        log::trace!("[SEND] -> S2 ");
                        let c2 = HandShark2::async_from(stream).await;
                        log::trace!("[RECEIVE] -> C2 time1:{:?} time2{:?}", c2.time1, c2.time2);
                        self.receive_handshark_state = Some(HandSharkState::C2(c2));
                    }
                    HandSharkState::C2(c2) => {
                        // 比对数据
                        assert_eq!(
                            c2.random_echo,
                            self.send_handshark_state.as_ref().unwrap().random_data
                        );
                        log::info!("[HANDSHARK SUCCESS]");
                        break;
                    }
                    _ => {
                        unreachable!()
                    }
                },
                None => {
                    let c0 = HandShark0::async_from(stream).await;
                    log::trace!("[RECEIVE]-> C0 version:{:?}", c0.version);
                    self.receive_handshark_state = Some(HandSharkState::C0(c0));
                }
            }
        }
        Ok(())
    }
}

pub async fn accpect_rtmp(mut stream: TcpStream) -> Result<()> {
    let mut rtmp_ctx = RtmpCtx::default();
    rtmp_ctx.handle_hand_check(&mut stream).await?;
    Ok(())
}
