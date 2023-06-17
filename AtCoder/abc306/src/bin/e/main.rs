use std::{cmp::Reverse, collections::BTreeSet};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        q: usize,
        xy: [(Usize1, i64); q],
    };

    let mut a = vec![0; n];
    let mut high = BTreeSet::new();
    let mut low = BTreeSet::new();
    for i in 0..n {
        if i < k {
            high.insert((Reverse(a[i]), i));
        } else {
            low.insert((Reverse(a[i]), i));
        }
    }
    let mut ans = 0;
    for (i, new) in xy {
        let old = a[i];
        if high.contains(&(Reverse(old), i)) {
            if let Some((Reverse(low_max), j)) = low.iter().next().copied() {
                if new >= low_max {
                    high.remove(&(Reverse(old), i));
                    high.insert((Reverse(new), i));
                    ans -= old;
                    ans += new;
                } else {
                    high.remove(&(Reverse(old), i));
                    low.remove(&(Reverse(low_max), j));
                    high.insert((Reverse(low_max), j));
                    low.insert((Reverse(new), i));
                    ans -= old;
                    ans += low_max;
                }
            } else {
                high.remove(&(Reverse(old), i));
                high.insert((Reverse(new), i));
                ans -= old;
                ans += new;
            }
        } else if low.contains(&(Reverse(old), i)) {
            let (Reverse(high_min), j) = high.iter().last().copied().unwrap();
            if new <= high_min {
                low.remove(&(Reverse(old), i));
                low.insert((Reverse(new), i));
            } else {
                low.remove(&(Reverse(old), i));
                high.remove(&(Reverse(high_min), j));
                low.insert((Reverse(high_min), j));
                high.insert((Reverse(new), i));
                ans -= high_min;
                ans += new;
            }
        } else {
            unreachable!();
        }
        a[i] = new;
        println!("{}", ans);
    }
}
