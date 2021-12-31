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

    // for n in 0..=100 {
    //     eprintln!("{} {}", n, (0..=n).fold(0, |acc, x| acc ^ x));
    // }

    let (a, b) = scan!((u64, u64));
    let mut ans = f(b);
    if a >= 1 {
        ans ^= f(a - 1);
    }
    println!("{}", ans);
}

fn f(n: u64) -> u64 {
    match n % 4 {
        0 => n,
        1 => 1,
        2 => n + 1,
        3 => 0,
        _ => unreachable!(),
    }
}
