use ext_gcd::ext_gcd;
use proconio::input;

fn main() {
    input! {
        x: i64,
        y: i64,
    };

    // b * x + (-a) * y = 1
    let (b, a, g) = ext_gcd(x, y);
    assert_ne!(g, 0);
    if 2 % g != 0 {
        println!("-1");
        return;
    }
    let (a, b) = match g {
        -2 | 2 => (-a, b),
        -1 | 1 => (-a * 2, b * 2),
        _ => unreachable!(),
    };
    assert_eq!(
        (i128::from(b) * i128::from(x) - i128::from(a) * i128::from(y)).abs(),
        2
    );
    const L: i64 = 1_000_000_000_000_000_000;
    if -L <= a && a <= L && -L <= b && b <= L {
        println!("{} {}", a, b);
    } else {
        println!("-1");
    }
}
