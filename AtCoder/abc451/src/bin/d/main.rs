use std::{collections::HashSet, iter::successors};

use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    const L: usize = 1_000_000_000;
    let t = successors(Some(1), |t| {
        let next = t * 2;
        if next <= L { Some(next) } else { None }
    })
    .collect::<Vec<_>>();

    let mut sets = vec![HashSet::new(); 11];
    sets[0] = HashSet::from([0]);
    for k in 1..=10 {
        let mut new_set = HashSet::new();
        for l in 0..k {
            for &s in &sets[l] {
                for &t in &t {
                    if keta(t) as usize == k - l {
                        new_set.insert(s * 10_usize.pow(keta(t)) + t);
                    }
                }
            }
        }
        sets[k] = new_set;
    }

    let mut values = sets.into_iter().skip(1).flatten().collect::<Vec<_>>();
    values.sort_unstable();

    let ans = values[n - 1];
    println!("{}", ans);
}

fn keta(x: usize) -> u32 {
    assert!(x > 0);

    let mut x = x;
    let mut k = 0;
    while x > 0 {
        x /= 10;
        k += 1;
    }
    k
}
