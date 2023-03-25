use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    };

    let mut f = HashMap::new();
    for x in a {
        *f.entry(x).or_insert(0) += 1;
    }

    let mut ans = 0;
    for (_, v) in f {
        ans += v / 2;
    }
    println!("{}", ans);
}
