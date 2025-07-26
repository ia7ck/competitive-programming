use proconio::{input, marker::Usize1};
use itertools::{repeat_n, Itertools};

fn main() {
    input! {
        n: usize,
        k: usize,
        x: Usize1,
        s: [String; n],
    };

    let mut all = Vec::new();
    for a in repeat_n(0..n, k).multi_cartesian_product() {
        let mut t = Vec::new();
        for i in a {
            t.push(s[i].clone());
        }
        all.push(t.join(""));
    }

    all.sort_unstable();
    let ans = all[x].clone();
    println!("{}", ans);
}
