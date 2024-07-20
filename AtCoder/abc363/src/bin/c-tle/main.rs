use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    };

    let mut ans = 0;
    for s in s.iter().permutations(n).unique() {
        let mut ok = true;
        for w in s.windows(k) {
            if w.iter().zip(w.iter().rev()).all(|(c1, c2)| c1 == c2) {
                ok = false;
                break;
            }
        }
        if ok {
            ans += 1;
        }
    }
    println!("{}", ans);
}
