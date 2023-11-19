use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    };

    let mut xor = 0;
    for x in &a {
        xor ^= x;
    }
    if xor != 0 {
        println!("-1");
        return;
    }

    let mut counter = HashMap::new();
    for &x in &a {
        *counter.entry(x).or_insert(0) += 1;
    }
    if let Some(m) = a.iter().filter(|x| counter[x] % 2 == 1).max() {
        let k = m - 1;
        println!("{}", k);
    } else {
        println!("0");
    }
}
