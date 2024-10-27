use std::collections::{hash_map::Entry, HashMap};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        queries: [Chars; n],
    };

    let mut root = TrieNode::new();
    for s in queries {
        let ans = root.add(&s);
        println!("{}", ans);
    }
}

#[derive(Debug)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    dist_to_leaf: usize,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            dist_to_leaf: 0,
        }
    }

    fn add(&mut self, s: &[char]) -> usize {
        self._add(s, 0, s.len())
    }

    fn _add(&mut self, s: &[char], i: usize, acc: usize) -> usize {
        if i == s.len() {
            return acc;
        }

        let c = s[i];
        match self.children.entry(c) {
            Entry::Occupied(e) => {
                let new_acc = acc.min((s.len() - i - 1) + e.get().dist_to_leaf);
                let e = e.into_mut();
                e.dist_to_leaf = e.dist_to_leaf.min(s.len() - i - 1);
                e._add(s, i + 1, new_acc)
            }
            Entry::Vacant(e) => {
                let e = e.insert(TrieNode::new());
                e.dist_to_leaf = s.len() - i - 1;
                e._add(s, i + 1, acc)
            }
        }
    }
}
