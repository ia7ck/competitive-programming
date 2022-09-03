use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
    };

    let mut cum_sum = vec![0; n + 1];
    for i in 0..n {
        cum_sum[i+1] = cum_sum[i] + a[i];
    }
    let mut s = 0;
    for i in 0..m {
        s += a[i] * (i + 1) as i64;
    }
    let mut ans = s;
    for i in m..n {
        s -= cum_sum[i] - cum_sum[i - m];
        s += a[i] * m as i64;
        ans = ans.max(s);
    }
    println!("{}", ans);
}
