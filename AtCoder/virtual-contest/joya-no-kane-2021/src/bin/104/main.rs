use std::collections::VecDeque;

use input_i_scanner::InputIScanner;
use join::Join;

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

    let (n, q) = scan!((usize, usize));
    let mut g = vec![vec![]; n];
    for _ in 0..(n - 1) {
        let (a, b) = scan!((usize, usize));
        g[a - 1].push(b - 1);
        g[b - 1].push(a - 1);
    }
    let mut score = vec![0; n];
    for _ in 0..q {
        let (p, x) = scan!((usize, u64));
        score[p - 1] += x;
    }
    let mut que = VecDeque::new();
    que.push_back((0, 0));
    while let Some((cur, prev)) = que.pop_front() {
        for &nxt in &g[cur] {
            if nxt == prev {
                continue;
            }
            score[nxt] += score[cur];
            que.push_back((nxt, cur));
        }
    }
    println!("{}", score.iter().join(" "));
}
