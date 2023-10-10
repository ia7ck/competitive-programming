use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        slimes: [(u64, u64); n],
    };

    let mut slimes = slimes.into_iter().collect::<BTreeMap<_, _>>();
    let mut ans = 0;
    while let Some((&s, &c)) = slimes.iter().next() {
        if c / 2 >= 1 {
            *slimes.entry(s * 2).or_insert(0) += c / 2;
        }
        ans += c & 1;
        slimes.remove(&s);
    }
    println!("{}", ans);
}
