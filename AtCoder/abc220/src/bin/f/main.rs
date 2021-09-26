use input_i_scanner::{scan_with, InputIScanner};

fn dfs(i: usize, p: usize, g: &Vec<Vec<usize>>, size: &mut Vec<usize>, dp: &mut Vec<u64>) {
    size[i] = 1;
    for &j in &g[i] {
        if j == p {
            continue;
        }
        dfs(j, i, g, size, dp);
        size[i] += size[j];
        dp[i] += dp[j] + size[j] as u64;
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    let n = scan_with!(_i_i, usize);
    let mut g = vec![vec![]; n];
    for _ in 0..(n - 1) {
        let (u, v) = scan_with!(_i_i, (usize, usize));
        g[u - 1].push(v - 1);
        g[v - 1].push(u - 1);
    }
    let mut size = vec![0; n];
    let mut dp = vec![0; n];
    dfs(0, !0, &g, &mut size, &mut dp);
    let mut ans = vec![0; n];
    ans[0] = dp[0];
    use std::collections::VecDeque;
    let mut q = VecDeque::new();
    q.push_back((0, !0));
    while let Some((i, p)) = q.pop_front() {
        for &j in &g[i] {
            if j == p {
                continue;
            }
            ans[j] = ans[i] - size[j] as u64 + (n - size[j]) as u64;
            q.push_back((j, i));
        }
    }
    for ans in ans {
        println!("{}", ans);
    }
}
