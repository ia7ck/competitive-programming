use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    };

    let mut ai = Vec::new();
    for i in 0..n {
        ai.push((a[i], i));
    }
    ai.sort_by_key(|&(x, _)| Reverse(x));

    let (_, ans) = ai[1];
    println!("{}",ans + 1);
}
