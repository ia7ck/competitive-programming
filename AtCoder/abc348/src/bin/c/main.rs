use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        ac: [(u32, u32); n],
    };

    let mut min_by_color = HashMap::new();
    for (a, c) in ac {
        let e = min_by_color.entry(c).or_insert(a);
        *e = (*e).min(a);
    }
    let ans = min_by_color.values().max().unwrap();
    println!("{}", ans);
}
