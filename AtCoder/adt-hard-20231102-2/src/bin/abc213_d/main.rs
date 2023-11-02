use proconio::{input, marker::Usize1};

fn dfs(i: usize, p: usize, g: &Vec<Vec<usize>>, ans: &mut Vec<usize>) {
    ans.push(i);
    for &j in &g[i] {
        if j == p {
            continue;
        }
        dfs(j, i, g, ans);
        ans.push(i);
    }
}

fn main() {
    input! {
        n: usize,
        edges: [(Usize1, Usize1); n - 1],
    };

    let mut g = vec![vec![]; n];
    for (a, b) in edges {
        g[a].push(b);
        g[b].push(a);
    }

    for v in 0..n {
        g[v].sort();
    }

    let mut ans = Vec::new();
    dfs(0, usize::MAX, &g, &mut ans);
    println!(
        "{}",
        ans.iter()
            .map(|x| (x + 1).to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
