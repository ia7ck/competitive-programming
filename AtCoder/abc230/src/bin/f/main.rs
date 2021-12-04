use input_i_scanner::InputIScanner;
use std::collections::HashMap;

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

    let n = scan!(usize);
    let a = scan!(i64; n);

    let mut pos: Vec<usize> = (0..n).collect();
    {
        // 1 1 1 2 -1 -1 -1 1 -1 3 3
        let mut s = 0;
        let mut map = HashMap::new();
        for i in 0..n {
            s += a[i];
            if let Some(j) = map.get(&s) {
                pos[i] = *j;
            }
            map.insert(s, i);
        }
    }

    let m: u64 = 998244353;
    let mut dp = vec![0; n];
    dp[0] = 1;
    for i in 1..n {
        dp[i] = dp[i - 1] * 2 % m;
        if pos[i - 1] != i - 1 {
            dp[i] = (m + dp[i] - dp[pos[i - 1]]) % m;
        }
    }

    println!("{}", dp[n - 1]);
}
