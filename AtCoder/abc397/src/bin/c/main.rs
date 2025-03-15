use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut prefix = Vec::new();
    let mut set = HashSet::new();
    for &x in &a {
        set.insert(x);
        prefix.push(set.len());
    }

    let mut suffix = Vec::new();
    let mut set = HashSet::new();
    for &x in a.iter().rev() {
        set.insert(x);
        suffix.push(set.len());
    }
    suffix.reverse();

    let mut ans = 0;
    for i in 0..n {
        if i + 1 < n {
            ans = ans.max(prefix[i] + suffix[i + 1]);
        }
    }
    println!("{}", ans);
}
