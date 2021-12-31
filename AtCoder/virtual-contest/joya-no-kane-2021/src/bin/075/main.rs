use std::f64::consts::PI;

use input_i_scanner::InputIScanner;

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
    let (x, y) = scan!((f64, f64));
    let (xx, yy) = scan!((f64, f64));

    let (cx, cy) = ((x + xx) / 2.0, (y + yy) / 2.0);
    let (vx, vy) = (x - cx, y - cy);
    let theta = 2.0 * PI / n as f64;
    let (vx, vy) = (
        vx * theta.cos() - vy * theta.sin(),
        vx * theta.sin() + vy * theta.cos(),
    );
    println!("{} {}", cx + vx, cy + vy);
}
