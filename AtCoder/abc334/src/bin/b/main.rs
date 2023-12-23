use proconio::input;

fn main() {
    input! {
        a: i64,
        m: i64,
        l: i64,
        r: i64,
    };

    // l <= a + k * m <= r
    // l - a <= k * m <= r - a

    let ans = if r - a >= 0 && l - a >= 0 {
        (r - a) / m - (l - a) / m + i64::from((l - a) % m == 0)
    } else if r - a < 0 && l - a < 0 {
        (l - a).abs() / m - (r - a).abs() / m + i64::from((r - a) % m == 0)
    } else {
        (r - a) / m + (l - a).abs() / m + 1
    };
    println!("{}", ans);
}
