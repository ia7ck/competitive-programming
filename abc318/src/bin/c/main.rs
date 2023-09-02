use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        p: u64,
        mut f: [u64; n],
    };

    f.sort();
    f.reverse();
    let mut cum_sum = vec![0; n + 1];
    for i in 0..n {
        cum_sum[i + 1] = cum_sum[i] + f[i];
    }
    let mut ans = u64::MAX;
    for k in 0..=n {
        let pass = p * k as u64;
        let regular = cum_sum[n] - cum_sum[(d * k).min(n)];
        ans = ans.min(pass + regular);
    }
    println!("{}", ans);
}
