use std::collections::HashMap;

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

    let n = scan!(usize);
    let a = scan!(i64; n);

    let mut freq = HashMap::new();
    freq.insert(0, 1);
    let mut sum = 0;
    let mut ans = 0_u64;
    for &a in &a {
        sum += a;
        if let Some(v) = freq.get(&sum) {
            ans += v;
        }
        freq.entry(sum).or_insert(0);
        freq.entry(sum).and_modify(|e| {
            *e += 1;
        });
    }
    println!("{}", ans);
}
