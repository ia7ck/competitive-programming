use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        x: [Usize1; m],
        a: [usize; m],
    };

    if a.iter().sum::<usize>() != n {
        println!("-1");
        return;
    }

    let mut xa = x.into_iter().zip(a.into_iter()).collect::<Vec<_>>();
    xa.sort(); // by x

    if xa[0].0 != 0 {
        println!("-1");
        return;
    }

    let mut cum_sum = 0;
    for &(x, a) in &xa {
        if cum_sum < x {
            println!("-1");
            return;
        }
        cum_sum += a;
    }

    let mut ans = 0;
    let mut r = n;
    for &(x, a) in xa.iter().rev() {
        assert!(r >= a);
        ans += ((r - a) - x) * a;
        ans += (a - 1) * a / 2; // = 0 + 1 + ... + (a - 1)
        r -= a;
    }

    println!("{}", ans);
}
