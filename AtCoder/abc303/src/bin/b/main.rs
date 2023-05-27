use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[usize; n]; m],
    };

    let mut s = HashSet::new();
    for i in 0..m {
        for w in a[i].windows(2) {
            s.insert((w[0].min(w[1]), w[0].max(w[1])));
        }
    }

    let mut ans = n * (n - 1) / 2;
    for x in 1..=n {
        for y in (x + 1)..=n {
            if s.contains(&(x, y)) {
                ans -= 1;
            }
        }
    }
    println!("{}", ans);
}
