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

    let (s, t, x) = scan!((u32, u32, u32));
    let ans = if s < t {
        if x >= s && x < t {
            "Yes"
        } else {
            "No"
        }
    } else {
        if x >= t && x < s {
            "No"
        } else {
            "Yes"
        }
    };
    println!("{}", ans);
}
