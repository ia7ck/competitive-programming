use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut xy: [(usize, usize); n],
    };

    xy.sort_unstable_by_key(|&(x, _)| x);
    let mut y_set = BTreeSet::<usize>::new();
    let mut ans = 0;
    // x = 1, 2, ..., n
    for (_, y) in xy {
        if y_set.range(..y).last().is_none() {
            ans += 1;
        }

        y_set.insert(y);
    }

    println!("{}", ans);
}
