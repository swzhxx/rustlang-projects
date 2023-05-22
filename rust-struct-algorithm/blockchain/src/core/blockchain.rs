// use super::block::Block;

// // 创世区块 pre_hash
// const PRE_HASH: &str = "22caaf24ef0aea3522c13d133912d2b722caaf24ef0aea3522c13d133912d2b7";

// pub struct BlockChain {
//     pub blocks: Vec<Block>,
// }

// impl BlockChain {
//     pub fn new() -> Self {
//         BlockChain {
//             blocks: vec![Self::genesis_block()],
//         }
//     }

//     // 生成创世区块
//     fn genesis_block() -> Block {
//         Block::new("创世区块".to_string(), PRE_HASH.to_string())
//     }

//     // 添加区块，形成区块链
//     pub fn add_block(&mut self, data: String) {
//         // 获取前一个区块的哈希值
//         let pre_block = &self.blocks[self.blocks.len() - 1];
//         let pre_hash = pre_block.hash.clone();
//         // 构建新区块并加入链
//         let new_block = Block::new(data, pre_hash);
//         self.blocks.push(new_block);
//     }

//     // 打印区块信息
//     pub fn block_info(&self) {
//         for b in self.blocks.iter() {
//             println!("{:#?}", b);
//         }
//     }
// }

use bigint::U256;
use leveldb::database::Database;

use crate::utils::{
    bkey::BKey,
    serializer::{hash_u8, serialize},
};

use super::{bcdb::BlockChainDb, block::Block};

const CURR_BITS: u32 = 0x2100FFFF;
const SAVE_DIR: &str = "bc_db";
const PRE_HASH: &str = "22caaf24ef0aea3522c13d133912d2b722caaf24ef0aea3522c13d133912d2b7";

pub struct BlockChain {
    pub blocks: Vec<Block>,
    curr_bits: u32,
    blocks_db: Box<Database<BKey>>,
}

impl BlockChain {
    pub fn new() -> Self {
        let mut db = BlockChainDb::new(SAVE_DIR);
        let gensis = Self::genesis_block();
        Self::write_block(&mut db, &gensis);
        Self::write_tail(&mut db, &gensis);
        println!("New produced block saved!\n");

        BlockChain {
            blocks: vec![gensis],
            curr_bits: CURR_BITS,
            blocks_db: Box::new(db),
        }
    }

    fn genesis_block() -> Block {
        Block::new("创世区块".to_string(), PRE_HASH.to_string(), CURR_BITS)
    }

    pub fn add_block(&mut self, txs: String) {
        let pre_block = &self.blocks[self.blocks.len() - 1];
        let pre_hash = pre_block.hash.clone();
        let new_block = Block::new(txs, pre_hash, self.curr_bits);

        // 数据写入库
        Self::write_block(&mut self.blocks_db, &new_block);
        Self::write_tail(&mut (self.blocks_db), &new_block);

        println!("New produced block saved!\n");
        self.blocks.push(new_block);
    }

    fn write_block(db: &mut Database<BKey>, block: &Block) {
        let header_ser = serialize(&block.header);
        let mut hash_u: [u8; 32] = [0; 32];
        hash_u8(&header_ser, &mut hash_u);

        let key = BKey {
            val: U256::from(hash_u),
        };
        let val = serialize(&block);
        BlockChainDb::write_db(db, key, &val);
    }

    fn write_tail(mut db: &mut Database<BKey>, block: &Block) {
        let key = BKey {
            val: U256::from("tail".as_bytes()),
        };
        let val = serialize(&block.hash);
        BlockChainDb::write_db(&mut db, key, &val);
    }

    pub fn block_info(&self) {
        for b in self.blocks.iter() {
            println!("{:#?}", b);
        }
    }
}
