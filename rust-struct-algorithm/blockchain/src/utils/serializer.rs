use bincode;
use crypto::{digest::Digest, sha3::Sha3};
use serde::Serialize;
// 序列化数据
pub fn serialize<T>(value: &T) -> Vec<u8>
where
    T: Serialize + ?Sized,
{
    bincode::serialize(value).unwrap()
}

// 计算 value 哈希值并以 String 形式返回
pub fn hash_str(value: &[u8]) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input(value);
    hasher.result_str()
}

pub fn hash_u8(value: &[u8], mut out: &mut [u8]) {
    let mut hasher = Sha3::sha3_256();
    hasher.input(value);
    hasher.result(&mut out);
}
