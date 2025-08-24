use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [u64; n],
        mut b: [u64; n],
        queries: [(char, Usize1, u64); q],
    };

    let mut ans = a.iter().zip(&b).map(|(a, b)| a.min(b)).sum::<u64>();
    for (op, i, v) in queries {
        ans -= a[i].min(b[i]);
        if op == 'A' {
            a[i] = v;
        } else {
            b[i] = v;
        }
        ans += a[i].min(b[i]);
        println!("{}", ans);
    }
}
