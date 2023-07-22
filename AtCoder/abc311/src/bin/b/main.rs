use std::println;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        d: usize,
        s: [Chars; n],
    };

    let mut ans = 0;
    for l in 0..d {
        for r in l..d {
            let mut ok = true;
            'outer: for s in &s {
                for i in l..=r {
                    if s[i] == 'x' {
                        ok = false;
                        break 'outer;
                    }
                }
            }
            if ok {
                ans = ans.max(r - l + 1);
            }
        }
    }
    println!("{}", ans);
}
