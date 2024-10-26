use proconio::{input, marker::Usize1};
use union_find::UnionFind;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [Usize1; n],
    };

    let mut uf = UnionFind::new(n);
    for i in 0..n {
        uf.unite(i, p[i]);
    }

    let mut dp = vec![vec![0; n]; 32];
    for j in 0..n {
        dp[0][j] = p[j];
    }
    for i in 1..32 {
        for j in 0..n {
            dp[i][j] = dp[i - 1][dp[i - 1][j]];
        }
    }

    let mut ans = Vec::new();
    for j in 0..n {
        let len = mpow(2, k, uf.get_size(j));
        let mut x = j;
        for i in 0..32 {
            if len >> i & 1 == 1 {
                x = dp[i][x];
            }
        }
        ans.push(x);
    }

    println!(
        "{}",
        ans.iter()
            .map(|x| x + 1)
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

fn mpow(a: usize, x: usize, m: usize) -> usize {
    if x == 0 {
        1 % m
    } else if x % 2 == 0 {
        mpow(a * a % m, x / 2, m)
    } else {
        a * mpow(a, x - 1, m) % m
    }
}
