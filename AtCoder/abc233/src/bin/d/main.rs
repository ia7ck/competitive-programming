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

    let (n, k) = scan!((usize, i64));
    let a = scan!(i64; n);

    let mut prefix_sum_freq = HashMap::new();
    prefix_sum_freq.insert(0, 1);
    let mut s = 0;
    let mut ans = 0u64;
    for a in a {
        s += a;
        if let Some(f) = prefix_sum_freq.get(&(s - k)) {
            ans += f;
        }
        prefix_sum_freq
            .entry(s)
            .and_modify(|e| {
                *e += 1;
            })
            .or_insert(1);
    }
    println!("{}", ans);
}
