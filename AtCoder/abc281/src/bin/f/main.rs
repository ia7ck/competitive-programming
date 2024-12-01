use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    };

    let mut trie = BinaryTrie::new();
    for i in 0..n {
        trie.add(a[i]);
    }
    println!("{}", trie.query());
}

#[derive(Debug, Clone)]
struct Node {
    children: [Option<Box<Node>>; 2],
}

impl Node {
    fn new() -> Self {
        Self {
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
        for i in (0..u32::BITS).rev() {
            let index = (value >> i & 1) as usize;
            node = node.children[index].get_or_insert_with(|| Box::new(Node::new()));
        }
    }

    fn query(&self) -> u32 {
        fn f(node: &Node, k: u32) -> u32 {
            if k == 0 {
                0
            } else {
                match &node.children {
                    [None, None] => 0,
                    [None, Some(one)] => f(one, k - 1),
                    [Some(zero), None] => f(zero, k - 1),
                    [Some(zero), Some(one)] => {
                        let zero = f(zero, k - 1);
                        let one = f(one, k - 1);
                        (1 << (k - 1)) + zero.min(one)
                    }
                }
            }
        }
        f(&self.root, u32::BITS)
    }
}
