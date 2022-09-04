use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    };

    let mut set = BTreeSet::new();
    for i in 0..n {
        set.insert((a[i], i));
    }

    let mut ans = 0;
    while set.len() >= 2 {
        let &(x, i) = set.iter().next().unwrap();
        let &(y, j) = set.iter().last().unwrap();
        set.remove(&(y, j));
        if y % x > 0 {
            set.insert((y % x, i));
        }
        ans += 1;
    }
    println!("{}", ans);
}
