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
        let a = scan!(i64; n);
        solve(n, a);
    }
}

fn solve(n: usize, a: Vec<i64>) {
    if n == 1 {
        println!("{}", a[0]);
        return;
    }

    let mut a = a;
    a.sort();
    let ans = a.windows(2).fold(a[0], |acc, w| acc.max(w[1] - w[0]));
    println!("{}", ans);
}
