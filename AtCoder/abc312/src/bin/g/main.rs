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

fn solve(i: usize, p: usize, g: &Vec<Vec<usize>>, size: &Vec<usize>, acc: usize, ans: &mut Vec<usize>) {
    // 頂点 i を根にして解く
    let mut children = vec![acc];
    for &j in &g[i] {
        if j == p {
            continue;
        }
        children.push(size[j]);
    }
    let mut dp1 = 0;
    let mut dp2 = 0;
    let mut dp3 = 0;
    for &c in &children {
        dp3 += dp2 * c;
        dp2 += dp1 * c;
        dp1 += c;
    }
    // eprintln!("i = {}, acc = {}, dp3 = {}, dp2 = {}, dp1 = {}", i, acc, dp3, dp2, dp1);
    ans[i] = dp3;

    // 再帰的に解く
    for &j in &g[i] {
        if j == p {
            continue;
        }
        solve(j, i, g, size, acc + size[i] - size[j], ans);
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

    let mut size = vec![0; n];
    dfs(0, std::usize::MAX, &g, &mut size);

    // eprintln!("size = {:?}", size);

    let mut ans = vec![0; n];
    solve(0, std::usize::MAX, &g, &size, 0, &mut ans);

    let mut total = 0;
    for ans in ans {
        total += ans;
    }
    println!("{}", total);
}
