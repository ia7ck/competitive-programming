use proconio::{input, marker::Usize1};

fn dfs(i: usize, g: &Vec<Vec<usize>>, seen: &mut Vec<bool>) {
    if seen[i] {
        return;
    }
    seen[i] = true;
    for &j in &g[i] {
        dfs(j, g, seen);
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
    };

    let mut g = vec![vec![]; n];
    for (u, v) in edges {
        g[u].push(v);
        g[v].push(u);
    }

    let mut ans = 0;
    let mut seen = vec![false; n];
    for i in 0..n {
        if seen[i] == false {
            ans += 1;
            dfs(i, &g, &mut seen);
        }
    }

    println!("{}", ans);
}
