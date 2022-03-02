use anyhow::Result;
use tokio::{net::TcpStream, stream};

use crate::{
    rtmp::protocol::handcheck::{HandCheck1, HandCheck2},
    util::{AsyncFrom, AsyncWriteByte},
};

use self::handcheck::{HandCheck0, HandCheckState};
pub mod handcheck;

#[derive(Debug, Default)]
struct RtmpCtx {
    receive_handcheck_state: Option<HandCheckState>,
    send_handcheck_state: Option<HandCheck1>,
}

impl RtmpCtx {
    async fn handle_hand_check<'a>(&mut self, stream: &'a mut TcpStream) -> anyhow::Result<()> {
        loop {
            match &self.receive_handcheck_state {
                Some(receive_handcheck_state) => match receive_handcheck_state {
                    HandCheckState::c0(_) => {
                        let c1 = HandCheck1::async_from(stream).await;
                        self.receive_handcheck_state = Some(HandCheckState::c1(c1));
                    }
                    HandCheckState::c1(c1) => {
                        let s2 = HandCheck2 {
                            time1: c1.time,
                            time2: self.send_handcheck_state.as_ref().unwrap().time,
                            random_echo: c1.random_data.clone(),
                        };
                    }
                    HandCheckState::c2(_) => break,
                    _ => {
                        unreachable!()
                    }
                },
                None => {
                    let c0 = HandCheck0::async_from(stream).await;
                    self.receive_handcheck_state = Some(HandCheckState::c0(c0));
                    let s0 = HandCheck0::default();
                    s0.async_write_byte(stream).await;
                    // self.send_handcheck_state = Some(HandCheckState::s0(s0));
                    let s1 = HandCheck1::default();
                    s1.async_write_byte(stream).await;
                    self.send_handcheck_state = Some(s1);
                }
            }
        }
        todo!()
    }
}

pub async fn accpect_rtmp(stream: TcpStream) -> Result<()> {
    todo!()
}
