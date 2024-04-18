use proconio::{input, marker::Usize1};

const P: usize = 1_000_000_000 + 7;

fn main() {
    input! {
        n: usize,
        edges: [(Usize1, Usize1); n - 1],
    }

    let mut fact = vec![1; n + 1];
    for i in 2..=n {
        fact[i] = fact[i - 1] * i % P;
    }
    let mut inv_fact = vec![1; n + 1];
    for i in 2..=n {
        inv_fact[i] = mpow(fact[i], P - 2);
    }
    for i in 1..=n {
        assert_eq!(fact[i] * inv_fact[i] % P, 1);
    }

    let mut g = vec![vec![]; n];
    for (u, v) in edges {
        g[u].push(v);
        g[v].push(u);
    }
    for i in 0..n {
        let mut dp = vec![1; n];
        let mut size = vec![0; n];
        dfs(i, usize::MAX, &g, &mut dp, &mut size, &fact, &inv_fact);
        println!("{}", dp[i]);
    }
}

fn mpow(a: usize, x: usize) -> usize {
    if x == 0 {
        1
    } else if x % 2 == 0 {
        mpow(a * a % P, x / 2)
    } else {
        a * mpow(a, x - 1) % P
    }
}

fn dfs(
    i: usize,
    p: usize,
    g: &Vec<Vec<usize>>,
    dp: &mut Vec<usize>,
    size: &mut Vec<usize>,
    fact: &Vec<usize>,
    inv_fact: &Vec<usize>,
) {
    for &j in &g[i] {
        if j == p {
            continue;
        }
        dfs(j, i, g, dp, size, fact, inv_fact);
        let choose =
            fact[size[i] + (size[j] + 1)] * inv_fact[size[i]] % P * inv_fact[size[j] + 1] % P;
        dp[i] *= dp[j] * choose % P;
        dp[i] %= P;
        size[i] += size[j] + 1;
    }
}
