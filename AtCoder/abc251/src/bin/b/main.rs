use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        w: usize,
        a: [usize; n],
    };

    let mut set = HashSet::new();
    for i in 0..n {
        set.insert(a[i]);
        for j in (i + 1)..n {
            set.insert(a[i] + a[j]);
            for k in (j + 1)..n {
                set.insert(a[i] + a[j] + a[k]);
            }
        }
    }

    let ans = set.into_iter().filter(|&s| s <= w).count();
    println!("{}", ans);
}
