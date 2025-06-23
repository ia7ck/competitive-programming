use std::{
    cell::RefCell,
    collections::{hash_map::Entry, HashMap},
    rc::{Rc, Weak},
};

use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        n: usize,
        q: usize,
    };

    let root = Rc::new(RefCell::new(TrieNode::new()));
    let mut pc = vec![Rc::clone(&root); n];
    let mut server = Rc::clone(&root);

    for _ in 0..q {
        input! {
            op: u8,
        };
        if op == 1 {
            input! {
                p: Usize1,
            };

            pc[p] = Rc::clone(&server);
        } else if op == 2 {
            input! {
                p: Usize1,
                s: Chars,
            };

            for c in s {
                pc[p] = insert(Rc::clone(&pc[p]), c);
            }
        } else {
            input! {
                p: Usize1,
            };

            server = Rc::clone(&pc[p]);
        }
    }

    let mut ans = Vec::<char>::new();
    let mut node = Rc::clone(&server);
    while !Rc::ptr_eq(&node, &root) {
        let data = node.borrow().data;
        ans.push(data);
        node = Rc::clone(&node)
            .borrow()
            .parent
            .as_ref()
            .unwrap()
            .upgrade()
            .unwrap();
    }
    ans.reverse();
    println!("{}", ans.iter().collect::<String>());
}

#[derive(Debug)]
struct TrieNode {
    data: char,
    parent: Option<Weak<RefCell<TrieNode>>>,
    children: HashMap<char, Rc<RefCell<TrieNode>>>,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            data: '?',
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
                data: c,
                parent: Some(Rc::downgrade(&node)),
                children: HashMap::new(),
            };
            Rc::clone(e.insert(Rc::new(RefCell::new(new_node))))
        }
    }
}
