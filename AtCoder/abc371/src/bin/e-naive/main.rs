use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut ans = 0;
    for i in 0..n {
        for j in i..n {
            let set = a[i..=j].iter().collect::<HashSet<_>>();
            ans += set.len();
        }
    }
    println!("{}", ans);
}
