use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut f = vec![0; n + 1];
    for &a in &a {
        f[a] += 1;
    }
    let mut ans = 0;
    for i in 1..=n {
        if f[i] >= 1 {
            let same = f[i] * (f[i] - 1) / 2;
            let other = n - f[i];
            ans += same * other;
        }
    }

    println!("{}", ans);
}
