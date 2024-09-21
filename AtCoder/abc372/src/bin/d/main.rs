use std::collections::BTreeSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        h: [Usize1; n],
    };

    let mut ans = Vec::new();
    let mut set = BTreeSet::<usize>::new();
    for &h in h.iter().rev() {
        ans.push(set.len());
        let small = set.range(0..h).copied().collect::<Vec<_>>();
        for x in small {
            set.remove(&x);
        }
        set.insert(h);
    }

    ans.reverse();
    for i in 0..n {
        print!("{}", ans[i]);
        if i + 1 < n {
            print!(" ");
        } else {
            print!("\n");
        }
    }
}
