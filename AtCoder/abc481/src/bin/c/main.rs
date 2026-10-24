use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        q: usize,
        queries: [String; q],
    };

    let mut map = HashMap::<String, usize>::new();
    for s in queries {
        let len = map.len();
        let v = map.entry(s).or_insert(len + 1);
        println!("{}", v);
    }
}
