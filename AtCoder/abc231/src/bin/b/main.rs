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
    let mut freq = HashMap::new();
    for _ in 0..n {
        let s = scan!(String);
        freq.entry(s)
            .and_modify(|e| {
                *e += 1;
            })
            .or_insert(0);
    }
    let mut s_freq: Vec<(String, usize)> = freq.into_iter().collect();
    s_freq.sort_by_key(|(_, freq)| *freq);
    s_freq.reverse();
    let (ans, _) = &s_freq[0];
    println!("{}", ans);
}
