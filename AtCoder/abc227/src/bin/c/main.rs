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

    let n = scan!(u64);
    let mut ans = 0u64;
    for a in 1..=n {
        if a.saturating_mul(a) > n {
            break;
        }
        for b in a..=n {
            if a.saturating_mul(b).saturating_mul(b) > n {
                break;
            }
            ans += n / a / b - b + 1;
        }
    }
    println!("{}", ans);
}
