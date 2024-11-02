use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        edges: [(Usize1, Usize1); n - 1],
    };

    let mut deg = vec![0; n];
    let mut g = vec![vec![]; n];
    for &(u, v) in &edges {
        deg[u] += 1;
        deg[v] += 1;
        g[u].push(v);
        g[v].push(u);
    }

    let mut ans = 0;
    let mut seen = vec![false; n];
    for i in 0..n {
        if seen[i] {
            continue;
        }
        if deg[i] == 3 {
            seen[i] = true;
            let mut d3 = vec![i];
            dfs(i, &g, &deg, &mut seen, &mut d3);
            let mut d2 = Vec::new();
            for &u in &d3 {
                for &v in &g[u] {
                    if deg[v] == 2 {
                        d2.push(v);
                    }
                }
            }
            d2.sort();
            d2.dedup();
            let l = d2.len();
            if l >= 2 {
                ans += l * (l - 1) / 2;
            }
        }
    }

    println!("{}", ans);
}

fn dfs(
    i: usize,
    g: &Vec<Vec<usize>>,
    deg: &Vec<usize>,
    seen: &mut Vec<bool>,
    acc: &mut Vec<usize>,
) {
    for &j in &g[i] {
        if seen[j] {
            continue;
        }
        if deg[j] == 3 {
            seen[j] = true;
            acc.push(j);
            dfs(j, g, deg, seen, acc);
        }
    }
}
