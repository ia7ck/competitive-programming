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
    let mut q = vec![0; n];
    for (i, &x) in p.iter().enumerate() {
        q[x] = i;
    }
    let mut ans = std::i64::MIN;
    for i in 0..n {
        let mut cur = i;
        let mut s = 0;
        let mut cycle = false;
        let mut len: usize = 0;
        for _ in 0..k {
            cur = p[cur];
            s += c[cur];
            ans = std::cmp::max(ans, s);
            len += 1;
            if cur == i {
                cycle = true;
                break;
            }
        }
        if cycle {
            let mut a = k / len;
            if a >= 2 {
                a -= 2;
            }
            s = s * a as i64;
            cur = i;
            let mut t = 0;
            for _ in 0..(k - a * len) {
                cur = p[cur];
                t += c[cur];
                ans = std::cmp::max(ans, s + t);
            }
        }
    }
    println!("{}", ans);
}
