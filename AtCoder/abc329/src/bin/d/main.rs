use std::{collections::BTreeSet, cmp::Reverse};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Usize1; m],
    };

    let mut count = vec![0; n];
    let mut set = BTreeSet::new();
    for i in 0..n {
        set.insert((Reverse(0), i));
    }
    for a in a {
        let exist = set.remove(&(Reverse(count[a]), a));
        assert!(exist);
        count[a] += 1;
        let new = set.insert((Reverse(count[a]), a));
        assert!(new);
        let (_, ans) = set.iter().next().unwrap().clone();
        println!("{}", ans + 1);
    }
}
