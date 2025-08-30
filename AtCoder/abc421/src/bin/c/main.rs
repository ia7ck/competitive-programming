use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    };

    let rev_s = s.iter().rev().copied().collect::<Vec<_>>();
    let ans = solve(s).min(solve(rev_s));
    println!("{}", ans);
}

// ABAB...AB
fn solve(s: Vec<char>) -> usize {
    let mut a = 0;
    let mut b = 0;
    let mut res = 0;
    for c in s {
        if c == 'A' {
            if b == 0 {
                a += 1;
            } else {
                res += b;
                b -= 1;
            }
        } else {
            if a == 0 {
                b += 1;
            } else {
                res += a - 1;
                a -= 1;
            }
        }
    }
    res
}
