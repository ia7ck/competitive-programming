use proconio::input;
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        _n: usize,
        q: usize,
    };

    let mut follows = HashMap::<u32, HashSet<u32>>::new();
    for _ in 0..q {
        input! {
            op: u8,
            a: u32,
            b: u32,
        };
        if op == 1 {
            follows.entry(a).or_insert(HashSet::new()).insert(b);
        } else if op == 2 {
            follows.entry(a).or_insert(HashSet::new()).remove(&b);
        } else {
            let mut yes = false;
            if let Some(f_a) = follows.get(&a) {
                if let Some(f_b) = follows.get(&b) {
                    if f_a.contains(&b) && f_b.contains(&a) {
                        yes = true;
                        println!("Yes");
                    }
                }
            }
            if !yes {
                println!("No");
            }
        }
    }
}
