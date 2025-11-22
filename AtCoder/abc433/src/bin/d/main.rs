use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: u64,
        a: [u64; n],
    };

    let len = a.iter().map(|x| x.to_string().len()).collect::<Vec<_>>();
    let mut ans = 0;
    for l in 1..=10 {
        let mut counter = HashMap::new();
        for &x in &a {
            *counter
                .entry(x * 10_u64.pow(l as u32) % m)
                .or_insert(0_usize) += 1;
        }
        for (i, &y) in a.iter().enumerate() {
            if len[i] == l {
                if let Some(c) = counter.get(&((m - y % m) % m)) {
                    ans += c;
                }
            }
        }
    }
    println!("{}", ans);
}
