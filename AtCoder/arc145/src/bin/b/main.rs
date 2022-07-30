use proconio::input;

fn main() {
    input! {
        n: u64,
        a: u64,
        b: u64,
    };

    let ans = if n < a {
        0
    } else if a < b {
        // a, a + 1, ..., n
        n - (a - 1)
    } else {
        // a, a + 1, ..., a + b - 1
        // 2a, 2a + 1, ..., 2a + b - 1
        let k = n / a;
        b * (k - 1) + (n.min(k * a + b - 1) - (k * a - 1))
    };
    println!("{}", ans);
}
