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

    let (n, w) = scan!((usize, u64));
    let mut ab = scan!((u64, u64); n);

    ab.sort_by_key(|&(a, _)| a);
    ab.reverse();
    let mut w = w;
    let mut ans = 0;
    for (a, b) in ab {
        if b <= w {
            ans += a * b;
            w -= b;
        } else {
            ans += a * w;
            break;
        }
    }
    println!("{}", ans);
}
