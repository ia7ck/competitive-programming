use scanner_proc_macro::insert_scanner;
use std::collections::VecDeque;

#[insert_scanner]
fn main() {
    let (n, m) = scan!((usize, usize));
    let mut g = vec![vec![]; n];
    for _ in 0..m {
        let (u, v) = scan!((usize, usize));
        g[u - 1].push(v - 1);
        g[v - 1].push(u - 1);
    }

    let inf = std::usize::MAX;
    let mut dp = vec![vec![inf; 1 << n]; n];
    let mut que = VecDeque::new();
    for v in 0..n {
        dp[v][0] = 0;
        dp[v][1 << v] = 1;
        que.push_back((v, 1 << v));
    }

    while let Some((u, bits)) = que.pop_front() {
        for &v in &g[u] {
            let nxt = bits ^ (1 << v);
            if dp[v][nxt] == inf {
                dp[v][nxt] = dp[u][bits] + 1;
                que.push_back((v, nxt));
            }
        }
    }

    let mut ans = 0;
    for bits in 0..(1 << n) {
        let mut mn = inf;
        for v in 0..n {
            mn = mn.min(dp[v][bits]);
        }
        assert_ne!(mn, inf);
        ans += mn;
    }
    println!("{}", ans);
}

// 4---2---3---1---5
//     |       |
//     +-------+