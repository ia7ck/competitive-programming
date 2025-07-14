use std::collections::BTreeSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        mut lrc: [(Usize1, Usize1, u64); q],
    };

    lrc.sort_unstable_by_key(|&(_, _, c)| c);

    let mut ans = 0;
    // i -- (i+1) が非連結
    let mut set = (0..(n - 1)).collect::<BTreeSet<_>>();
    for (l, r, c) in lrc {
        match set.range(l..r).next() {
            Some(&i) => {
                ans += c * 2;
                set.remove(&i);
                let mut i = i + 1;
                while let Some(&j) = set.range(i..r).next() {
                    ans += c;
                    set.remove(&j);
                    i = j + 1;
                }
            }
            None => {
                ans += c;
            }
        }
    }

    if set.is_empty() {
        println!("{}", ans);
    } else {
        println!("-1");
    }

}
