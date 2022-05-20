use std::collections::HashSet;

use divisors::Divisors;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    };

    let set: HashSet<usize> = a.into_iter().collect();
    let mut open = vec![false; n + 1];
    let mut ans = n;
    for i in (1..=n).rev() {
        let yes = if set.contains(&i) {
            !open[i]
        } else {
            open[i]
        };
        if yes {
            ans -= 1;
            for d in i.divisors() {
                open[d] = !open[d];
            }
        }
    }
    println!("{}", ans);
}
