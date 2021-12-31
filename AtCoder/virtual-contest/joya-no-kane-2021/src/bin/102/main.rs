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

    let mut n = scan!(u64);

    let mut ans = Vec::new();
    for i in (1..=n).rev() {
        let s = i * (i - 1) / 2; // 1 + 2 + ... + (i - 1)
        if s < n {
            ans.push(i);
            n -= i;
        }
    }
    for ans in ans {
        println!("{}", ans);
    }
}
