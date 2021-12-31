use std::collections::VecDeque;

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
    let mut g = vec![vec![]; n];
    for i in 0..(n - 1) {
        let (a, b) = scan!((usize, usize));
        g[a - 1].push((b - 1, i));
        g[b - 1].push((a - 1, i));
    }
    let mut ans = vec![0; n - 1];
    let mut que = VecDeque::new();
    que.push_back((0, 0, 0));
    while let Some((cur, prev, ng_color)) = que.pop_front() {
        let mut color = 1;
        for &(nxt, i) in &g[cur] {
            if nxt == prev {
                continue;
            }
            if color == ng_color {
                color += 1;
            }
            ans[i] = color;
            que.push_back((nxt, cur, color));
            color += 1;
        }
    }
    let mut k = 0;
    for &ans in &ans {
        k = k.max(ans);
        assert_ne!(ans, 0);
    }
    println!("{}", k);
    for ans in ans {
        println!("{}", ans);
    }
}
