use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
        q: usize,
    };

    let mut before = HashMap::new();
    let mut after = HashMap::new();
    for i in 0..n {
        let before_ = if i == 0 { None } else { Some(a[i - 1]) };
        let after_ = if i + 1 == n { None } else { Some(a[i + 1]) };
        before.insert(a[i], before_);
        after.insert(a[i], after_);
    }
    for _ in 0..q {
        input! {
            op: u8,
        };
        if op == 1 {
            input! {
                x: u32,
                y: u32,
            };
            let z = after[&x];
            match z {
                Some(z) => {
                    after.insert(x, Some(y));
                    before.insert(y, Some(x));
                    after.insert(y, Some(z));
                    before.insert(z, Some(y));
                }
                None => {
                    after.insert(x, Some(y));
                    before.insert(y, Some(x));
                    after.insert(y, None);
                }
            }
        } else {
            input! {
                x: u32,
            };

            let s = before[&x];
            let t = after[&x];

            match (s, t) {
                (Some(s), Some(t)) => {
                    after.insert(s, Some(t));
                    before.insert(t, Some(s));
                }
                (Some(s), None) => {
                    after.insert(s, None);
                }
                (None, Some(t)) => {
                    before.insert(t, None);
                }
                (None, None) => {
                    unreachable!();
                }
            }

            before.remove(&x);
            after.remove(&x);
        }
    }

    let (first, _) = before.iter().find(|(_, before)| before.is_none()).unwrap();
    let mut x = *first;
    let mut ans = vec![x];
    while let Some(y) = after[&x] {
        ans.push(y);
        x = y;
    }
    println!(
        "{}",
        ans.iter()
            .map(|&x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
