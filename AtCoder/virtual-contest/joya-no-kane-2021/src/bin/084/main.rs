use binary_search::BinarySearch;
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
    let a = scan!(u32; n);
    let b = scan!(u32; n);
    let c = scan!(u32; n);

    let mut a = a;
    let mut c = c;
    a.sort();
    c.sort();
    let mut ans = 0;
    for b in b {
        let a = a.lower_bound(&b);
        let c = n - c.upper_bound(&b);
        ans += a * c;
    }
    println!("{}", ans);
}
