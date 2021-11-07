use input_i_scanner::InputIScanner;
use std::collections::HashSet;

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    macro_rules! scan {
        (($($t: ty),+)) => {
            ($(scan!($t)),+)
        };
        ($t: ty) => {
            _i_i.scan::<$t>() as $t
        };
        (($($t: ty),+); $n: expr) => {
            std::iter::repeat_with(|| scan!(($($t),+))).take($n).collect::<Vec<_>>()
        };
        ($t: ty; $n: expr) => {
            std::iter::repeat_with(|| scan!($t)).take($n).collect::<Vec<_>>()
        };
    }

    let n = scan!(usize);
    let points = scan!((i64, i64); n);

    let mut set = HashSet::new();
    for &(x, y) in &points {
        for &(xx, yy) in &points {
            if (x, y) == (xx, yy) {
                continue;
            }
            let dx = x - xx;
            let dy = y - yy;
            let g = gcd(dx, dy);
            set.insert((dx / g.abs(), dy / g.abs()));
        }
    }
    println!("{}", set.len());
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
