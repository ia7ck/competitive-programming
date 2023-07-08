use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        c: [String; n],
        d: [String; m],
        q: u64,
        p: [u64; m],
    };

    let mut map = HashMap::new();
    for i in 0..m {
        map.insert(d[i].clone(), p[i]);
    }
    let mut ans = 0;
    for i in 0..n {
        ans += map.get(&c[i]).unwrap_or(&q);
    }
    println!("{}", ans);
}
