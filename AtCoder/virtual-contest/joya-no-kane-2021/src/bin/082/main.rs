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

    let n = scan!(u64);

    let mut ans = 0;
    let mut i = 1;
    while i <= n {
        // i: 1, 11, 111, 1111, ...
        let mut j = i;
        let mut k = i + 1;
        while j <= n {
            // i: 111
            // j: 111, 1110, 11100, 111000, ...
            // k: 112, 1120, 11200, 112000, ...
            ans += k.min(n) - j;
            j *= 10;
            k *= 10;
        }
        i = i * 10 + 1;
    }
    println!("{}", ans + f(n));
}

fn f(n: u64) -> u64 {
    n.to_string().chars().take_while(|&ch| ch == '1').count() as u64
}
