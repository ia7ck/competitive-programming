use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        p: [usize; n],
    };

    let mut q = vec![0.0; n + 1];
    q[0] = 1.0;
    for p in p {
        let p = p as f64 / 100.0;
        let mut new_q = vec![0.0; q.len()];
        for i in 0..q.len() {
            new_q[i] += q[i] * (1.0 - p);
            if i + 1 < new_q.len() {
                new_q[i + 1] += q[i] * p;
            }
        }
        q = new_q;
    }

    let mut dp = vec![0.0; x + n + 1];
    for i in (0..x).rev() {
        let den = 1.0 - q[0];
        let mut num = q[0];
        for j in 1..=n {
            num += (dp[i + j] + 1.0) * q[j];
        }
        dp[i] = num / den;
    }

    println!("{}", dp[0]);
}
