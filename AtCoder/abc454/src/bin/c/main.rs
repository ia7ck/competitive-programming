use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
    };

    let mut g = vec![vec![]; n];
    for (a, b) in edges {
        g[a].push(b);
    }

    let mut seen = vec![false; n];
    dfs(0, &g, &mut seen);

    let ans = seen.iter().filter(|&&seen| seen).count();

    println!("{}", ans);
}

fn dfs(i: usize, g: &Vec<Vec<usize>>, seen: &mut Vec<bool>) {
    if seen[i] {
        return;
    }

    seen[i] = true;

    for &j in &g[i] {
        dfs(j, g, seen);
    }
}
