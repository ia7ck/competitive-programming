use proconio::input;

fn main() {
    input! {
        s: u64,
        a: u64,
        b: u64,
        x: u64,
    };

    let mut ans = 0;
    let mut t = 0;
    while t < x {
        if t + a > x {
            ans += s * (x - t);
            // t = x;
            break;
        }
        ans += s * a;
        t += a;

        if t + b > x {
            // t = x;
            break;
        }
        t += b;
    }

    println!("{}", ans);
}
