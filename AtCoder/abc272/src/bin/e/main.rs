use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
    };

    let mut b = vec![HashSet::new(); m + 1];
    for i in 0..n {
        let mut j = if a[i] < 0 {
            ((-a[i]) as usize + i) / (i + 1)
        } else if a[i] as usize <= n {
            1
        } else {
            m + 1
        };
        let mut x = (a[i] + ((i + 1) * j) as i64) as usize;
        while j <= m && x <= n {
            b[j].insert(x);
            j += 1;
            x += i + 1;
        }
    }
    for i in 1..=m {
        let mut ans = 0;
        while b[i].contains(&ans) {
            ans += 1;
        }
        println!("{}", ans);
    }
}
