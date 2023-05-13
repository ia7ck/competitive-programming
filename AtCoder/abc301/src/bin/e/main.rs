use std::collections::VecDeque;

use grid_search::around;
use proconio::{input, marker::Chars};

const INF: usize = std::usize::MAX;
fn bfs((si, sj): (usize, usize), a: &Vec<Vec<char>>) -> Vec<Vec<usize>> {
    let mut dist = vec![vec![INF; a[0].len()]; a.len()];
    dist[si][sj] = 0;
    let mut que = VecDeque::new();
    que.push_back((si, sj));
    let dirs = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    while let Some((i, j)) = que.pop_front() {
        for (ni, nj) in around(i, j)
            .y_range(0..a.len())
            .x_range(0..a[i].len())
            .directions(&dirs)
        {
            if a[ni][nj] == '#' {
                continue;
            }
            if dist[ni][nj] == INF {
                dist[ni][nj] = dist[i][j] + 1;
                que.push_back((ni, nj));
            }
        }
    }
    dist
}

fn main() {
    input! {
        h: usize,
        w: usize,
        t: usize,
        a: [Chars; h],
    };

    let (mut si, mut sj) = (0, 0);
    let (mut gi, mut gj) = (0, 0);
    let mut candies = Vec::new();
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == 'S' {
                si = i;
                sj = j;
            }
            if a[i][j] == 'G' {
                gi = i;
                gj = j;
            }
            if a[i][j] == 'o' {
                candies.push((i, j));
            }
        }
    }

    let dist_s = bfs((si, sj), &a);
    let dist_g = bfs((gi, gj), &a);
    let mut dist_s_candy = Vec::new();
    let mut dist_g_candy = Vec::new();
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == 'o' {
                dist_s_candy.push(dist_s[i][j]);
                dist_g_candy.push(dist_g[i][j]);
            }
        }
    }

    if dist_s[gi][gj] > t {
        println!("-1");
        return;
    }

    let mut dist_candy_candy = vec![vec![INF; candies.len()]; candies.len()];
    for start in 0..candies.len() {
        let dist = bfs(candies[start], &a);
        let mut d = Vec::new();
        for i in 0..h {
            for j in 0..w {
                if a[i][j] == 'o' {
                    d.push(dist[i][j]);
                }
            }
        }
        dist_candy_candy[start] = d;
    }

    let mut candy_max = 0;
    for start in 0..candies.len() {
        let mut dp = vec![vec![INF; 1 << candies.len()]; candies.len()];
        dp[start][1 << start] = 0;
        for bits in 0..(1 << candies.len()) {
            for u in 0..candies.len() {
                if bits >> u & 1 == 0 {
                    continue;
                }
                if dp[u][bits] == INF {
                    continue;
                }
                for v in 0..candies.len() {
                    if bits >> v & 1 == 1 {
                        continue;
                    }
                    let next_bits = bits ^ (1 << v);
                    dp[v][next_bits] =
                        dp[v][next_bits].min(dp[u][bits].saturating_add(dist_candy_candy[u][v]));
                }
            }
        }
        for bits in 0..(1 << candies.len()) {
            for v in 0..candies.len() {
                let d = dist_s_candy[start]
                    .saturating_add(dp[v][bits])
                    .saturating_add(dist_g_candy[v]);
                if d <= t {
                    let c = bits.count_ones();
                    candy_max = candy_max.max(c);
                }
            }
        }
    }

    println!("{}", candy_max);
}
