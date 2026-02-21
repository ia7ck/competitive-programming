use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    };

    let mut dp = HashMap::<u32, usize>::new();
    for a in a {
        let v = match dp.get(&(a - 1)) {
            Some(v) => v + 1,
            None => 1,
        };
        dp.entry(a)
            .and_modify(|e| {
                *e = (*e).max(v);
            })
            .or_insert(v);
    }

    let ans = dp.values().max().unwrap();
    println!("{}", ans);
}
