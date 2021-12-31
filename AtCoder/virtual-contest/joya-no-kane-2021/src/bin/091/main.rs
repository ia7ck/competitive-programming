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

    let (n, c) = scan!((usize, u64));
    let mut timeline = Vec::new();
    let mut enter: HashMap<u32, Vec<u64>> = HashMap::new();
    let mut leave: HashMap<u32, Vec<u64>> = HashMap::new();
    for _ in 0..n {
        let (a, b, w) = scan!((u32, u32, u64));
        timeline.push(a);
        timeline.push(b);
        enter.entry(a).or_insert(Vec::new()).push(w);
        leave.entry(b).or_insert(Vec::new()).push(w);
    }
    timeline.push(0);
    timeline.push(1_000_000_000 + 1);
    timeline.sort();
    timeline.dedup();
    let mut tot = 0;
    let mut ans = 0;
    for window in timeline.windows(2) {
        let (t1, t2) = (window[0], window[1]);
        // t1+1, t1+2, ..., t2-1
        ans += tot.min(c) * (t2 - t1 - 1) as u64;

        if let Some(enter) = enter.get(&t2) {
            for &w in enter {
                tot += w;
            }
        }

        // t2
        ans += tot.min(c);

        if let Some(leave) = leave.get(&t2) {
            for &w in leave {
                tot -= w;
            }
        }
    }
    println!("{}", ans);
}
