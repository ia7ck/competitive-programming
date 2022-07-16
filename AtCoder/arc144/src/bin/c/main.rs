use std::collections::BTreeSet;

use join::Join;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };

    if n / 2 < k {
        println!("-1");
        return;
    }

    let mut ans = Vec::new();
    let mut set = BTreeSet::new();
    for i in (k + 1)..=n {
        set.insert(i);
    }
    let mut used = vec![false; n + 1];
    let first = set.iter().next().copied().unwrap();
    used[first] = true;
    ans.push(first);
    set.remove(&first);
    for i in 2..=n {
        if i > k {
            if !used[i - k] {
                let new = set.insert(i - k);
                assert_eq!(new, true);
            }
        }
        if i + k - 1 <= n {
            set.remove(&(i + k - 1));
        }
        if n - k < i + k && i + k <= n {
            if !used[i + k] {
                used[i + k] = true;
                ans.push(i + k);
                set.remove(&(i + k));
                continue;
            }
        }
        let min = set.iter().next().copied().unwrap();
        used[min] = true;
        ans.push(min);
        set.remove(&min);
    }

    println!("{}", ans.iter().join(" "));
}
