use crate::core::blockchain;

mod core;
mod utils;

fn main() {
    println!("--------------------Mine Info--------------------");
    let mut bc = blockchain::BlockChain::new();
    let tx = "0xabcd -> 0xabce: 5 btc".to_string();
    bc.add_block(tx);
    let tx = "0xabcd -> 0xabcf: 2.5 btc".to_string();
    bc.add_block(String::from(tx));
    println!("----------------Block Info------------------");
    bc.block_info();
}
