use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        d: i64,
        a: [i64; n],
    };

    let mut ans = 0_usize;
    let mut set = BTreeSet::new();
    set.insert(a[0]);
    let mut i = 0;
    for j in 1..n {
        loop {
            let prev = set.range(..=a[j]).last().copied().unwrap_or(i64::MIN);
            let next = set.range(a[j]..).next().copied().unwrap_or(i64::MAX);
            if prev + d <= a[j] && a[j] + d <= next {
                break;
            } else {
                set.remove(&a[i]);
                i += 1;
            }
        }
        set.insert(a[j]);
        ans += j - i;
    }

    // (j, j)
    ans += n;

    println!("{}", ans);
}
