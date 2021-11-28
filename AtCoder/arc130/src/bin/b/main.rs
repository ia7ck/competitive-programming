use input_i_scanner::InputIScanner;
use join::Join;
use std::collections::HashSet;

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

    let (h, w, c, q) = scan!((u64, u64, usize, usize));
    let queries = scan!((u8, u64, usize); q);

    let mut rows = HashSet::new();
    let mut cols = HashSet::new();
    let mut ans = vec![0; c + 1];
    for (t, n, c) in queries.into_iter().rev() {
        match t {
            1 => {
                if rows.contains(&n) {
                    continue;
                }
                ans[c] += w - cols.len() as u64;
                rows.insert(n);
            }
            2 => {
                if cols.contains(&n) {
                    continue;
                }
                ans[c] += h - rows.len() as u64;
                cols.insert(n);
            }
            _ => unreachable!(),
        }
    }
    println!("{}", ans[1..=c].iter().join(" "));
}
