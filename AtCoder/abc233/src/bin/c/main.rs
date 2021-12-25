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

    let (n, x) = scan!((usize, u64));
    let a: Vec<Vec<u64>> = (0..n)
        .map(|_| {
            let l = scan!(usize);
            scan!(u64; l)
        })
        .collect();

    let mut dp = HashMap::<u64, u64>::new();
    dp.insert(1, 1);
    for a in a {
        let mut nxt = HashMap::new();
        for (k, v) in dp {
            for &a in &a {
                nxt.entry(k.saturating_mul(a))
                    .and_modify(|e| *e += v)
                    .or_insert(v);
            }
        }
        dp = nxt;
    }
    if let Some(ans) = dp.get(&x) {
        println!("{}", ans);
    } else {
        println!("0");
    }
}
