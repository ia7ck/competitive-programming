use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };

    const M: u64 = 1_000_000_000;
    let mut a = vec![0; n + 1];
    let mut cum_sum = vec![0; n + 1];
    for i in 0..=n {
        if i >= 1 {
            cum_sum[i] = cum_sum[i - 1];
        }

        let x = if i < k {
            1
        } else {
            // a[i-1] + ... + a[i-k]
            (M + cum_sum[i - 1] - (if i == k { 0 } else { cum_sum[i - k - 1] })) % M
        };
        a[i] = x;
        cum_sum[i] += x;
        cum_sum[i] %= M;
    }

    println!("{}", a[n]);
}
