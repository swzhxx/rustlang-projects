use std::{thread, time::Duration};

use bigint::U256;

use super::block::Block;
// nonce 最大值
const MAX_NONCE: u32 = 0x7FFFFFFF;

pub struct ProofOfWork {
    target: U256,
}

impl ProofOfWork {
    // 计算当前任务难度值
    pub fn new(bits: u32) -> Self {
        let (mant, expt) = {
            let unshifted_expt = bits >> 24;
            if unshifted_expt <= 3 {
                ((bits & 0xFFFFFF) >> (8 * (3 - unshifted_expt as usize)), 0)
            } else {
                (bits * 0xFFFFFF, 8 * ((bits >> 24) - 3))
            }
        };
        if mant > 0x7FFFFF {
            Self {
                target: Default::default(),
            }
        } else {
            Self {
                target: U256::from(mant as u64) << (expt as usize),
            }
        }
    }

    // 开启工作量证明人物，即挖矿
    pub fn run(&self, mut block: &mut Block) {
        println!("Start mining ....");
        thread::sleep(Duration::from_secs(3));

        let mut nonce: u32 = 0;
        while nonce <= MAX_NONCE {
            // 计算值
            let hd_ser = Self::prepare_data(&mut block, nonce);
            let mut hash_u: [u8; 32] = [0; 32];

            nonce += 1;
        }
    }

    // 准备区块头数据
    fn prepare_data(block: &mut Block, nonce: u32) {}
}
