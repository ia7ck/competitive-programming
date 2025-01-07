use mod_int::ModInt998244353;
use proconio::{input, marker::Usize1};
use strongly_connected_components::strongly_connected_components;

type Mint = ModInt998244353;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Usize1; n],
    };

    let mut edges = Vec::new();
    for i in 0..n {
        edges.push((i, a[i]));
    }
    let comps = strongly_connected_components(n, &edges);

    let mut comp_id = vec![0; n];
    for (id, comp) in comps.iter().enumerate() {
        for &i in comp {
            comp_id[i] = id;
        }
    }

    let mut in_deg = vec![0; n];
    let mut g = vec![vec![]; n];
    for &(s, t) in &edges {
        if comp_id[s] == comp_id[t] {
            continue;
        }
        in_deg[s] += 1;
        g[t].push(s);
    }

    let mut ans = Mint::new(1);
    let mut dp = vec![vec![Mint::new(0); m + 1]; n];
    for comp in comps {
        if in_deg[comp[0]] == 0 {
            for &r in &comp {
                solve(r, m, &g, &mut dp);
            }
            let mut sum = Mint::new(0);
            for k in 1..=m {
                let mut prod = Mint::new(1);
                for &r in &comp {
                    prod *= dp[r][k];
                }
                sum += prod;
            }
            ans *= sum;
        }
    }

    println!("{}", ans.val());
}

fn solve(i: usize, m: usize, g: &Vec<Vec<usize>>, dp: &mut Vec<Vec<Mint>>) {
    for &j in &g[i] {
        solve(j, m, g, dp);
    }

    if cfg!(debug_assertions) {
        for k in 1..=m {
            dp[i][k] = Mint::new(1);
            for &j in &g[i] {
                let mut sum = Mint::new(0);
                for l in 1..=k {
                    sum += dp[j][l];
                }
                dp[i][k] *= sum;
            }
        }
    } else {
        let mut sums = Vec::new();
        for &j in &g[i] {
            let mut sum = vec![Mint::new(0); m + 1];
            for k in 1..=m {
                sum[k] = sum[k - 1] + dp[j][k];
            }
            sums.push(sum);
        }
        for k in 1..=m {
            dp[i][k] = Mint::new(1);
            for sum in &sums {
                dp[i][k] *= sum[k];
            }
        }
    }
}
