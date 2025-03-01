use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    };

    let mut last = HashMap::new();
    let mut ans = usize::MAX;
    for i in 0..n {
        if let Some(p) = last.get(&a[i]) {
            ans = ans.min(i - p + 1);
        }
        last.insert(a[i], i);
    }

    if ans == usize::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
