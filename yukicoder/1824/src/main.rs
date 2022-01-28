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
        let n = scan!(usize);
        solve(n);
    }
}

fn solve(n: usize) {
    // n    0
    // n-1  1*0
    // n-2  2*1
    // n-3  3*2
    // ...
    // 1    (n-1)*(n-2)
    // n    (n-1)*(n-2)
    // n-1  (n-2)*(n-3)
    // ...
    // 2    1*0
    // 1    0

    // let ans = (1..n).map(|i| i * (i - 1)).sum::<usize>() * 2;
    let ans = ((n - 1) * ((n - 1) + 1) * ((n - 1) * 2 + 1) / 6 - (n - 1) * n / 2) * 2;
    println!("{}", ans);
}
