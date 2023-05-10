// 字典树定义
#[derive(Default)]
struct Trie {
    root: Node,
}

// 节点
#[derive(Default)]
struct Node {
    end: bool,
    children: [Option<Box<Node>>; 26], // 字符节点列表
}

impl Trie {
    fn new() -> Self {
        Self::default()
    }
    // 单词插入
    fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        // 逐个字符插入
        for c in word.as_bytes() {
            let index = (c - b'a') as usize;
            let next = &mut node.children[index];
            node = next.get_or_insert_with(Box::<Node>::default);
        }
        node.end = true
    }

    fn search(&self, word: &str) -> bool {
        self.word_node(word).map_or(false, |n| n.end)
    }

    // 判断是否存在以某个前缀开头的单词
    fn start_with(&self, prefix: &str) -> bool {
        self.word_node(prefix).is_some()
    }

    // 前缀字符串
    fn word_node(&self, wps: &str) -> Option<&Node> {
        let mut node = &self.root;
        for c in wps.as_bytes() {
            let index = (c - b'a') as usize;
            match &node.children[index] {
                None => return None,
                Some(next) => node = next.as_ref(),
            }
        }
        Some(node)
    }
}

#[cfg(test)]
mod test {
    use super::Trie;

    #[test]
    fn text_trie() {
        let mut trie = Trie::new();
        trie.insert("box");
        trie.insert("insert");
        trie.insert("apple");
        trie.insert("appeal");
        assert_eq!(trie.search("apple"), true);
        assert_eq!(trie.search("apples"), false);
        assert_eq!(trie.start_with("ins"), true);
        assert_eq!(trie.start_with("ina"), false);
    }
}
