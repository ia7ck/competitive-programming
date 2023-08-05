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

fn solve(i: usize, p: usize, g: &Vec<Vec<usize>>, size: &Vec<usize>, acc: usize, path: &mut Vec<usize>) {
    let mut children = vec![acc];
    for &j in &g[i] {
        if j == p {
            continue;
        }
        children.push(size[j]);
    }
    let mut dp1 = 0;
    let mut dp2 = 0;
    for &c in &children {
        dp2 += dp1 * c;
        dp1 += c;
    }
    path[i] = dp2;

    for &j in &g[i] {
        if j == p {
            continue;
        }
        solve(j, i, g, size, acc + size[i] - size[j], path);
    }
}

fn main() {
    input! {
        n: usize,
        edges: [(Usize1, Usize1); n - 1],
    };

    if n <= 2 {
        println!("0");
        return;
    }

    let mut g = vec![vec![]; n];
    for (a, b) in edges {
        g[a].push(b);
        g[b].push(a);
    }

    let mut size = vec![0; n];
    dfs(0, std::usize::MAX, &g, &mut size);

    let mut path = vec![0; n];
    solve(0, std::usize::MAX, &g, &size, 0, &mut path);

    let mut ans = n * (n - 1) * (n - 2) / 6;
    for p in path {
        ans -= p;
    }
    println!("{}", ans);
}
