use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::{collections::BTreeMap, fmt::Debug};
const DEFAULT_REPLICAS: usize = 10;

#[derive(Clone, Debug)]
// 环上节点
struct Node {
    host: &'static str,
    ip: &'static str,
    port: u16,
}

impl ToString for Node {
    fn to_string(&self) -> String {
        format!("{}:{}", self.ip.to_string(), self.port.to_string())
    }
}
// 环
struct Ring<T>
where
    T: Clone + ToString + Debug,
{
    replicas: usize,
    ring: BTreeMap<u64, T>,
}

fn hash<T>(val: &T) -> u64
where
    T: Hash,
{
    let mut hasher = DefaultHasher::new();
    val.hash(&mut hasher);
    hasher.finish()
}

impl<T> Ring<T>
where
    T: Clone + ToString + Debug,
{
    fn new() -> Self {
        Self::with_capacity(DEFAULT_REPLICAS)
    }

    fn with_capacity(replicas: usize) -> Self {
        Ring {
            replicas,
            ring: BTreeMap::new(),
        }
    }

    // 批量插入结点
    fn add_multi(&mut self, nodes: &[T]) {
        if !nodes.is_empty() {
            for node in nodes.iter() {
                self.add(node)
            }
        }
    }

    fn add(&mut self, node: &T) {
        for i in 0..self.replicas {
            let key = hash(&format!("{}{}", node.to_string(), i.to_string()));
            self.ring.insert(key, node.clone());
        }
    }

    // 批量删除结点
    fn remove_multi(&mut self, nodes: &[T]) {
        if !nodes.is_empty() {
            for node in nodes.iter() {
                self.remove(node);
            }
        }
    }

    fn remove(&mut self, node: &T) {
        assert!(!self.ring.is_empty());
        for i in 0..self.replicas {
            let key = hash(&format!("{}{}", node.to_string(), i.to_string()));
            self.ring.remove(&key);
        }
    }

    // 查询结点
    fn get(&self, key: u64) -> Option<&T> {
        if self.ring.is_empty() {
            return None;
        } else {
            let mut keys = self.ring.keys();
            keys.find(|&k| k >= &key)
                .and_then(|k| self.ring.get(k))
                .or(keys.nth(0).and_then(|x| self.ring.get(x)))
        }
    }
}

#[cfg(test)]
mod test {
    use super::{hash, Node, Ring};

    #[test]
    fn test_conshahs() {
        let replica = 3;
        let mut ring = Ring::with_capacity(replica);
        let node = Node {
            host: "localhost",
            ip: "127.0.0.1",
            port: 23,
        };
        ring.add(&node);
        for i in 0..replica {
            let key = hash(&format!("{}{}", node.to_string(), i.to_string()));
            let res = ring.get(key);
            assert_eq!(node.host, res.unwrap().host);
        }
        println!("{:?}", &node);
        ring.remove(&node);
    }
}
