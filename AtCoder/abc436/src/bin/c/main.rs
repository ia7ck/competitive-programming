use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        _n: usize,
        m: usize,
        rc: [(usize, usize); m],
    };

    let mut ans = 0;
    let mut cover = HashSet::new();
    for (r, c) in rc {
        let set = [(r, c), (r + 1, c), (r, c + 1), (r + 1, c + 1)];
        if set.iter().all(|&(r, c)| !cover.contains(&(r, c))) {
            ans += 1;
            for (r, c) in set {
                cover.insert((r, c));
            }
        }
    }
    println!("{}", ans);
}
