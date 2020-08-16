extern crate proconio;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut p: [usize; n],
        c: [i64; n],
    }
    let p = p.iter().map(|x| x - 1).collect::<Vec<_>>();
    let mut ans = std::i64::MIN;
    for i in 0..n {
        let mut cur = i;
        let mut s = 0;
        for _ in 0..k {
            cur = p[cur];
            s += c[cur];
            ans = std::cmp::max(ans, s);
        }
    }
    println!("{}", ans);
}
