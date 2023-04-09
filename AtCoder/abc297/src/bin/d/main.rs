use proconio::input;

fn main() {
    input! {
        mut a: u64,
        mut b: u64,
    };

    let mut ans = 0;
    loop {
        if a < b {
            std::mem::swap(&mut a, &mut b);
        }
        if a % b == 0 {
            ans += a / b - 1;
            a -= (a / b - 1) * b;
            assert_eq!(a, b);
            break;
        } else {
            ans += a / b;
            a %= b;
        }
    }
    println!("{}", ans);
}
