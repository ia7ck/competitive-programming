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
        let a = scan!(u64; n);
        solve(n, a);
    }
}

fn solve(_n: usize, a: Vec<u64>) {
    let tot = a.iter().sum::<u64>();
    let max = a.iter().copied().max().unwrap();
    if tot % 3 == 0 && tot / 3 >= max {
        println!("Yes");
    } else {
        println!("No");
    }
}
