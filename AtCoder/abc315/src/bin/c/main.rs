use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
    };

    let mut s = vec![vec![]; n];
    for _ in 0..n {
        input! {
            f: Usize1,
            v: u64,
        };
        s[f].push(v);
    }

    for i in 0..n {
        s[i].sort();
        s[i].reverse();
    }
    let mut y = Vec::new();
    let mut ans = 0;
    for s in s {
        if s.is_empty() {
            continue;
        }
        y.push(s[0]);
        if s.len() >= 2 {
            ans = ans.max(s[0] + s[1] / 2);
        }
    }
    y.sort();
    y.reverse();
    if y.len() >= 2 {
        ans = ans.max(y[0] + y[1]);
    }

    println!("{}", ans);
}
