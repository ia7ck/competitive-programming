use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    };

    let mut b = Vec::new();
    let mut last = HashMap::<u32, isize>::new();
    for i in 0..n {
        b.push(last.get(&a[i]).copied().unwrap_or(-1));
        last.insert(a[i], (i + 1) as isize);
    }

    println!(
        "{}",
        b.iter()
            .map(|&x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
