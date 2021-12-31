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

    let (h, n) = scan!((u64, usize));
    let ab = scan!((u64, u64); n);

    macro_rules! chmin {
        ($a: expr, $b: expr) => {
            $a = std::cmp::min($a, $b);
        };
    }

    let mut dp = vec![std::u64::MAX; h as usize + 1];
    dp[0] = 0;
    for i in 0..h {
        if dp[i as usize] == std::u64::MAX {
            continue;
        }
        for &(a, b) in &ab {
            chmin!(dp[(i + a).min(h) as usize], dp[i as usize] + b);
        }
    }
    println!("{}", dp[h as usize]);
}
