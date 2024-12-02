use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [u32; n],
    };

    let mut trie = BinaryTrie::new();
    for i in 0..n {
        trie.add(a[i]);
    }

    let ans = trie.query(k * 2 + n - 1);
    println!("{}", ans);
}

#[derive(Debug, Clone)]
struct Node {
    size: usize,
    children: [Option<Box<Node>>; 2],
}

impl Node {
    fn new() -> Self {
        Self {
            size: 0,
            children: [None, None],
        }
    }
}

#[derive(Debug)]
struct BinaryTrie {
    root: Node,
}

impl BinaryTrie {
    fn new() -> Self {
        Self { root: Node::new() }
    }

    fn add(&mut self, value: u32) {
        let mut node = &mut self.root;
        node.size += 1;
        for i in (0..u32::BITS).rev() {
            let index = (value >> i & 1) as usize;
            node = node.children[index].get_or_insert_with(|| Box::new(Node::new()));
            node.size += 1;
        }
    }

    fn list(&self) -> Vec<(u32, usize)> {
        fn f(node: &Node, k: Option<u32>, x: u32, acc: &mut Vec<(u32, usize)>) {
            match k {
                None => {
                    acc.push((x, node.size));
                }
                Some(k) => {
                    let new_k = k.checked_sub(1);
                    if let Some(zero) = &node.children[0] {
                        f(zero, new_k, x, acc);
                    }
                    if let Some(one) = &node.children[1] {
                        f(one, new_k, x ^ (1 << k), acc);
                    }
                }
            }
        }
        let mut result = Vec::new();
        f(&self.root, Some(u32::BITS - 1), 0, &mut result);
        result
    }

    // ● xor ▲ を昇順に並べたときの k 番目 (0-indexed) の数を返す
    fn query(&self, k: usize) -> u32 {
        let mut nodes = Vec::new();
        for (x, l) in self.list() {
            nodes.push((&self.root, (x, l)));
        }
        let (mut count, mut result) = (0, 0);
        for i in (0..u32::BITS).rev() {
            // result の i ビット目を 0 にしたとき、result 以下が確定する組の個数
            let c = nodes
                .iter()
                .map(|(n, (x, l))| {
                    let index = (x >> i & 1) as usize;
                    match &n.children[index] {
                        None => 0,
                        Some(child) => child.size * l,
                    }
                })
                .sum::<usize>();

            let accept = count + c <= k;
            if accept {
                result ^= 1 << i;
                count += c;
            }
            nodes.retain_mut(|(n, (x, _))| {
                let index = if accept {
                    (*x >> i & 1) ^ 1
                } else {
                    *x >> i & 1
                };
                if let Some(ch) = &n.children[index as usize] {
                    *n = ch;
                    true
                } else {
                    false
                }
            });
        }
        result
    }
}
