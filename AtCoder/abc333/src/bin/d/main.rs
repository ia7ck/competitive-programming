use proconio::{input, marker::Usize1};

fn dfs(i: usize, p: usize, g: &Vec<Vec<usize>>, size: &mut Vec<usize>) {
    size[i] = 1;
    for &j in &g[i] {
        if j == p {
            continue;
        }
        dfs(j, i, g, size);
        size[i] += size[j];
    }
}

fn main() {
    input! {
        n: usize,
        edges: [(Usize1, Usize1); n - 1],
    };

    let mut g = vec![vec![]; n];
    for &(u, v) in &edges {
        g[u].push(v);
        g[v].push(u);
    }
    let mut size = vec![0; n];
    dfs(0, n, &g, &mut size);
    let mut child_size = Vec::new();
    for &j in &g[0] {
        child_size.push(size[j]);
    }
    let ans = child_size.iter().sum::<usize>() - child_size.iter().max().copied().unwrap() + 1;
    println!("{}", ans);
}
