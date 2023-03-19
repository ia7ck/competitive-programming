use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    };

    let mut o = 1;
    let mut set = BTreeSet::new();
    for _ in 0..q {
        input! {
            ev: u8,
        };
        if ev == 1 {
            assert!(o <= n);
            let new = set.insert(o);
            assert!(new);
            o += 1;
        } else if ev == 2 {
            input! {
                x: usize,
            };
            assert!(set.contains(&x));
            set.remove(&x);
        } else {
            let ans = set.iter().next().unwrap();
            println!("{}", ans);
        }
    }
}
