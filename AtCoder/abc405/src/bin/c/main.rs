use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    };

    let mut cum_sum = vec![0; n + 1];
    for i in 0..n {
        cum_sum[i + 1] = cum_sum[i] + a[i];
    }
    let mut ans = 0;
    for j in 0..n {
        ans += cum_sum[j] * a[j];
    }
    println!("{}", ans);
}
