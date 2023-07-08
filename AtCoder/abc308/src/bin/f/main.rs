use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        p: [u64; n],
        l: [u64; m],
        d: [u64; m],
    };

    let total = p.iter().sum::<u64>();
    let mut sub = 0;
    let mut set = BTreeSet::new();
    for i in 0..n {
        set.insert((p[i], i));
    }
    let mut ord = (0..m).collect::<Vec<_>>();
    ord.sort_by_key(|&i| (d[i], l[i]));
    ord.reverse();
    for o in ord {
        let (d, l) = (d[o], l[o]);
        if let Some(&(p, i)) = set.range(&(l, 0)..).next() {
            set.remove(&(p, i));
            sub += d;
        }
    }
    assert!(total >= sub);
    println!("{}", total - sub);
}
