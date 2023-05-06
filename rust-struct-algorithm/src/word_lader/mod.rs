use std::collections::HashMap;

use crate::graph_adjlist::Graph;

pub fn build_word_lader(words: &[String]) -> Graph<String> {
    let mut d: HashMap<String, Vec<String>> = HashMap::new();
    for word in words {
        for i in 0..word.len() {
            let bucket = (word[..i].to_string() + "_") + &word[i + 1..];
            let wd = word.to_string();
            if d.contains_key(&bucket) {
                d.get_mut(&bucket).unwrap().push(wd);
            } else {
                d.insert(bucket, vec![wd]);
            }
        }
    }

    let mut g = Graph::new();
    for bucket in d.keys() {
        for wd1 in &d[bucket] {
            for wd2 in &d[bucket] {
                if wd1 != wd2 {
                    g.add_edge(wd1, wd2, 1);
                }
            }
        }
    }
    g
}
