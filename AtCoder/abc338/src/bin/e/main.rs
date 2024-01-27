use std::collections::BTreeSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n],
    };

    #[derive(Debug)]
    enum E {
        In(usize),
        Out(usize),
    }
    let mut events = Vec::new();
    let mut to = vec![usize::MAX; n * 2];
    for (a, b) in ab {
        let (a, b) = (a.min(b), a.max(b));
        events.push(E::In(a));
        events.push(E::Out(b));
        to[a] = b;
    }
    events.sort_by_key(|e| match e {
        E::In(x) => *x,
        E::Out(y) => *y,
    });
    let mut set = BTreeSet::new();
    for e in events {
        match e {
            E::In(x) => {
                let y = to[x];
                assert_ne!(y, usize::MAX);
                if set.range(x..y).next().is_some() {
                    println!("Yes");
                    return;
                }
                set.insert(y);
            }
            E::Out(y) => {
                set.remove(&y);
            }
        }
    }
    println!("No");
}
