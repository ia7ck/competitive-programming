use proconio::input;

const P: usize = 998244353;

fn main() {
    input! {
        n: usize,
        m: usize,
    };

    let mut ans = 0;
    for i in 0..60 {
        if m >> i & 1 == 1 {
            let a = 2_usize.pow(i + 1);
            let b = a / 2;
            ans += (n + 1) / a * b;
            ans %= P;
            ans += ((n + 1) % a).saturating_sub(b);
            ans %= P;
        }
    }

    println!("{}", ans);
}
