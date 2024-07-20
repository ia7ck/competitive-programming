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

use std::cell::{Ref, RefCell};
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    children: [Option<Rc<RefCell<Node>>>; 2],
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
    root: Rc<RefCell<Node>>,
}

impl BinaryTrie {
    fn new() -> Self {
        Self {
            root: Rc::new(RefCell::new(Node::new())),
        }
    }

    fn add(&mut self, value: u32) {
        let mut node = Rc::clone(&self.root);
        for i in (0..u32::BITS).rev() {
            let index = (value >> i & 1) as usize;
            if node.borrow().children[index].is_none() {
                node.borrow_mut().children[index] = Some(Rc::new(RefCell::new(Node::new())));
            }
            let new = Rc::clone(&node.borrow().children[index].as_ref().unwrap());
            node = new;
        }
    }

    fn query(&self) -> u32 {
        fn f(node: Ref<Node>, k: u32) -> u32 {
            if k == 0 {
                0
            } else {
                match &node.children {
                    [None, None] => 0,
                    [None, Some(one)] => f(one.borrow(), k - 1),
                    [Some(zero), None] => f(zero.borrow(), k - 1),
                    [Some(zero), Some(one)] => {
                        let zero = f(zero.borrow(), k - 1);
                        let one = f(one.borrow(), k - 1);
                        (1 << (k - 1)) + zero.min(one)
                    }
                }
            }
        }
        f(self.root.borrow(), u32::BITS)
    }
}
