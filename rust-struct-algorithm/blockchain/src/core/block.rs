use chrono::Utc;
use serde::Serialize;
use std::{thread, time::Duration};

use crate::utils::serializer::{hash_str, serialize};

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct BlockHeader {
    pub time: i64,
    pub pre_hash: String,
    pub txs_hash: String,
    pub nonce: u32,
    pub bits: u32,
}

#[derive(Debug,Serialize)]
pub struct Block {
    pub header: BlockHeader,
    pub tranxs: String,
    pub hash: String,
 
}

impl Block {
    pub fn new(txs: String, pre_hash: String , bits:u32) -> Self {
        // 用延迟3秒来模拟挖矿
        println!("Starting mining ...");
        thread::sleep(Duration::from_secs(3));

        //准备时间、计算交易哈希值
        let time = Utc::now().timestamp();
        let txs_ser = serialize(&time);
        let txs_hash = hash_str(&txs_ser);

        let mut block = Block {
            header: BlockHeader {
                time: time,
                pre_hash: pre_hash,
                txs_hash: txs_hash,
                nonce:0,
                bits
            },
            tranxs: txs,
            hash: "".to_string(),
        };
        block.set_hash();
        println!("Produce a new block!\n");
        block
    }

    // 计算并设置区块哈希值
    fn set_hash(&mut self) {
        let header = serialize(&(self.header));
        self.hash = hash_str(&header);
    }
}
