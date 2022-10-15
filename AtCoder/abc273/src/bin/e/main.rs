use proconio::{fastout, input};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
struct Node {
    val: u32,
    next: Option<Rc<Node>>,
}

#[derive(Debug, Clone)]
struct Stack {
    head: Option<Rc<Node>>,
}

impl Stack {
    fn new() -> Self {
        Self { head: None }
    }

    fn top(&self) -> Option<u32> {
        self.head.as_ref().map(|head| head.val)
    }

    fn push(&self, x: u32) -> Self {
        let next = self.head.clone();
        Self {
            head: Some(Rc::new(Node { val: x, next })),
        }
    }

    fn pop(&self) -> Self {
        if let Some(head) = self.head.as_ref() {
            if let Some(next) = head.next.as_ref() {
                return Self {
                    head: Some(Rc::clone(next)),
                };
            }
        }
        Self { head: None }
    }
}

#[fastout]
fn main() {
    input! {
        q: usize,
    };
    let mut ans = Vec::new();
    let mut map = HashMap::new();
    let mut stack = Stack::new();
    for _ in 0..q {
        input! {
            op: String,
        };
        if op == "ADD" {
            input! {
                x: u32,
            };
            stack = stack.push(x);
        } else if op == "DELETE" {
            stack = stack.pop();
        } else if op == "SAVE" {
            input! {
                y: u32,
            };
            map.insert(y, stack.clone());
        } else if op == "LOAD" {
            input! {
                z: u32,
            };
            stack = map.get(&z).cloned().unwrap_or(Stack::new());
        } else {
            unreachable!();
        }
        if let Some(top) = stack.top() {
            ans.push(top as i32);
        } else {
            ans.push(-1);
        }
    }
    for i in 0..q {
        print!("{}", ans[i]);
        if i + 1 < q {
            print!(" ");
        }
    }
    println!();
}
