use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        strings: [String; n],
    };

    let mut freq = HashMap::<String, usize>::new();
    for s in strings {
        if let Some(f) = freq.get_mut(&s) {
            println!("{}({})", s, f);
            *f += 1;
        } else {
            println!("{}", s);
            freq.insert(s, 1);
        }
    }
}
