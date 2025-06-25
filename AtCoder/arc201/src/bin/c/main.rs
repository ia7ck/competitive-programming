use std::{
    cell::RefCell,
    collections::{hash_map::Entry, HashMap},
    rc::{Rc, Weak},
};

use mod_int::ModInt998244353;
use proconio::input;

type Mint = ModInt998244353;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    };

    let root = Rc::new(RefCell::new(TrieNode::default()));
    for s in s {
        let mut node = Rc::clone(&root);
        for c in s.chars() {
            node = insert(node, c);
        }

        node.borrow_mut().data.end = true;
        loop {
            let a = node.borrow().children.get(&'A').cloned();
            let b = node.borrow().children.get(&'B').cloned();
            let (new_num_end, new_dp) = match (a, b) {
                (None, None) => {
                    if node.borrow().data.end {
                        (1, Mint::new(1))
                    } else {
                        (0, Mint::new(0))
                    }
                }
                (a, b) => {
                    let new_num_end = usize::from(node.borrow().data.end)
                        + a.as_ref().map_or(0, |a| a.borrow().data.num_end)
                        + b.as_ref().map_or(0, |b| b.borrow().data.num_end);
                    let mut new_dp = a.map_or(Mint::new(0), |a| a.borrow().data.dp)
                        * b.map_or(Mint::new(0), |b| b.borrow().data.dp);
                    if node.borrow().data.end {
                        new_dp += Mint::new(2).pow((new_num_end - 1) as u32);
                    }
                    (new_num_end, new_dp)
                }
            };
            node.borrow_mut().data.num_end = new_num_end;
            node.borrow_mut().data.dp = new_dp;

            if Rc::ptr_eq(&node, &root) {
                break;
            }

            let parent = node.borrow().parent.as_ref().unwrap().upgrade().unwrap();
            node = parent;
        }

        let ans = root.borrow().data.dp.val();
        println!("{}", ans);
    }
}

#[derive(Debug)]
struct S {
    end: bool,
    num_end: usize, // # of end
    dp: Mint,       // この頂点からスタートして葉に辿りつけないようにendの頂点をon/offする場合の数
}

impl Default for S {
    fn default() -> Self {
        Self {
            end: false,
            num_end: 0,
            dp: Mint::new(0),
        }
    }
}

#[allow(unused)]
#[derive(Debug)]
struct TrieNode {
    data: S,
    c: char,
    parent: Option<Weak<RefCell<TrieNode>>>,
    children: HashMap<char, Rc<RefCell<TrieNode>>>,
}

impl Default for TrieNode {
    fn default() -> Self {
        Self {
            data: S::default(),
            c: '$',
            parent: None,
            children: HashMap::new(),
        }
    }
}

fn insert(node: Rc<RefCell<TrieNode>>, c: char) -> Rc<RefCell<TrieNode>> {
    match RefCell::borrow_mut(&node).children.entry(c) {
        Entry::Occupied(e) => Rc::clone(e.get()),
        Entry::Vacant(e) => {
            let new_node = TrieNode {
                data: S::default(),
                c,
                parent: Some(Rc::downgrade(&node)),
                children: HashMap::new(),
            };
            Rc::clone(e.insert(Rc::new(RefCell::new(new_node))))
        }
    }
}
