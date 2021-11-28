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

    let _n = scan!(usize);
    let s = scan!(String);

    let mut last = '?';
    let mut len = 0usize;
    let mut a = Vec::new();
    for ch in s.chars() {
        if last == ch {
            len += 1;
        } else {
            a.push(len);
            len = 1;
        }
        last = ch;
    }
    a.push(len);
    let mut ans = 0;
    for len in a {
        if len >= 2 {
            ans += len * (len - 1) / 2;
        }
    }
    println!("{}", ans);
}
