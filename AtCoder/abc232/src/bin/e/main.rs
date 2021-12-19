use input_i_scanner::InputIScanner;
use mod_int::ModInt998244353;

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

    type Mint = ModInt998244353;
    let zero = Mint::new(0);
    let one = Mint::new(1);

    let (h, w, k) = scan!((u32, u32, usize));
    let (x1, y1, x2, y2) = scan!((u32, u32, u32, u32));
    let (mut xy, mut ay, mut xb, mut ab) = match (x1 == x2, y1 == y2) {
        (true, true) => (one, zero, zero, zero),
        (true, false) => (zero, zero, one, zero),
        (false, true) => (zero, one, zero, zero),
        (false, false) => (zero, zero, zero, one),
    };
    for _ in 0..k {
        let (n_xy, n_ay, n_xb, n_ab) = {
            (
                ay + xb,
                xy * (h - 1) + ay * (h - 2) + ab,
                xy * (w - 1) + xb * (w - 2) + ab,
                ay * (w - 1) + xb * (h - 1) + ab * (h - 2) + ab * (w - 2),
            )
        };
        xy = n_xy;
        ay = n_ay;
        xb = n_xb;
        ab = n_ab;
    }
    println!("{}", xy.val());
}
