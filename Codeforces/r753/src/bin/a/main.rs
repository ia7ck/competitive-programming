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
        let alph = scan!(String);
        let s = scan!(String);
        solve(alph, s);
    }
}

fn solve(alph: String, s: String) {
    let alph = alph.as_bytes();
    let ans = s.as_bytes().windows(2).fold(0, |acc, w| {
        let start = alph.iter().position(|&c| c == w[0]).unwrap();
        let end = alph.iter().position(|&c| c == w[1]).unwrap();
        if start >= end {
            acc + (start - end)
        } else {
            acc + (end - start)
        }
    });
    println!("{}", ans);
}
