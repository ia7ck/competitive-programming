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

    let t = scan!(usize);
    for _ in 0..t {
        let (x, n) = scan!((i64, i64));
        solve(x, n);
    }
}

fn solve(x: i64, n: i64) {
    let ans = match (x.abs() % 2, n % 4) {
        (0, 0) => x,
        (0, 1) => x - n,
        (0, 2) => x + 1,
        (0, 3) => x + (n + 1),
        (1, 0) => x,
        (1, 1) => x + n,
        (1, 2) => x - 1,
        (1, 3) => x - (n + 1),
        _ => unreachable!(),
    };
    println!("{}", ans);
}

// 0
// -1
// 1
// 4
// 0
// -5
// 1
// 8
// 0
// -9
// 1
// 12
// 0
