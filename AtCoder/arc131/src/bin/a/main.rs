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

    let a = scan!(u64);
    let b = scan!(u64);

    let mut ans = b / 2;
    if b % 2 == 0 {
        ans = ans * 10;
    } else {
        ans = ans * 10 + 5;
    }
    ans *= 10_u64.pow(9);
    ans += a;
    assert!(ans < 1_000_000_000_000_000_000);
    println!("{}", ans);
    eprintln!("ans * 2 = {}", ans * 2);
}
