use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    };

    const P: u64 = 998244353;
    let mut len = vec![0; 11];
    let mut sum = 0;
    let mut ans = 0;
    for i in (0..n).rev() {
        for k in 1..=10 {
            ans += a[i] * 10_u64.pow(k) % P * len[k as usize] % P;
            ans %= P;
        }
        ans += sum;
        ans %= P;
        len[a[i].to_string().len()] += 1;
        sum += a[i];
        sum %= P;
    }
    println!("{}", ans);
}
