use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
        d: u64,
        e: u64,
        f: u64,
    };

    let mo: u64 = 998244353;
    let (a, b, c, d, e, f) = (a % mo, b % mo, c % mo, d % mo, e % mo, f % mo);
    let x = a * b % mo * c % mo;
    let y = d * e % mo * f % mo;
    let ans = (mo + x - y) % mo;
    println!("{}", ans);
}
