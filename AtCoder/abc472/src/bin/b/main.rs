use proconio::input;

fn main() {
    input! {
        n: usize,
        l: [u64; n],
    };

    let mut cumsum = vec![0; n + 1];
    for i in 0..n {
        cumsum[i + 1] = cumsum[i] + l[i];
    }
    let mut ans = u64::MAX;
    for i in 0..(n - 1) {
        let left = cumsum[i + 1];
        let right = cumsum[n] - cumsum[i + 1];
        ans = ans.min(left.abs_diff(right));
    }

    println!("{}", ans);
}
