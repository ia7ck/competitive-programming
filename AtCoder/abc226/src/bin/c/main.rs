use input_i_scanner::InputIScanner;
use std::collections::VecDeque;

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
    let mut tt = vec![0; n];
    let mut aa = vec![vec![]; n];
    for i in 0..n {
        let t = scan!(u64);
        tt[i] = t;
        let k = scan!(usize);
        if k == 0 {
            continue;
        }
        let a = scan!(usize; k);
        aa[i] = a;
    }

    let mut seen = vec![false; n];
    seen[n - 1] = true;
    let mut que = VecDeque::new();
    que.push_back(n - 1);
    while let Some(u) = que.pop_front() {
        for &v in &aa[u] {
            let v = v - 1;
            if seen[v] {
                continue;
            }
            seen[v] = true;
            que.push_back(v);
        }
    }
    let mut ans = 0;
    for i in 0..n {
        if seen[i] {
            ans += tt[i];
        }
    }
    println!("{}", ans);
}
