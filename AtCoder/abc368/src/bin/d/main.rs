use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        edges: [(Usize1, Usize1); n - 1],
        v: [Usize1; k],
    };

    let mut g = vec![vec![]; n];
    for (a, b) in edges {
        g[a].push(b);
        g[b].push(a);
    }
    let mut keep = vec![false; n];
    for v in v {
        keep[v] = true;
    }

    let mut size = vec![0; n];
    let mut count = vec![0; n];
    let mut ans = vec![0; n];
    dfs(0, usize::MAX, &g, &keep, &mut size, &mut count, &mut ans);

    let mut res = n;
    for i in 0..n {
        if count[i] >= k {
            res = res.min(ans[i]);
        }
    }
    println!("{}", res);
}

fn dfs(i: usize, p: usize, g: &Vec<Vec<usize>>, keep: &Vec<bool>, size: &mut Vec<usize>, count :&mut Vec<usize>, ans: &mut Vec<usize>) {
    for &j in &g[i] {
        if j == p {
            continue;
        }
        dfs(j, i, g, keep, size, count, ans);
        size[i] += size[j];
        count[i] += count[j];
        if count[j] >= 1 {
            ans[i] += ans[j];
        }
    }
    size[i] += 1;
    if keep[i] {
        count[i] += 1;
    }
    if keep[i] || ans[i] >= 1 {
        ans[i] += 1;
    }
}
