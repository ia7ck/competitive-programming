use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        _m: usize,
        s: Chars,
        t: Chars,
        q: usize,
        w: [Chars; q],
    };

    let s = s.into_iter().collect::<HashSet<_>>();
    let t = t.into_iter().collect::<HashSet<_>>();
    for w in w {
        let takahashi = w.iter().all(|c| s.contains(c));
        let aoki = w.iter().all(|c| t.contains(c));
        let ans = match (takahashi, aoki) {
            (true, false) => "Takahashi",
            (false, true) => "Aoki",
            (true, true) => "Unknown",
            _ => unimplemented!(),
        };
        println!("{}", ans);
    }
}
