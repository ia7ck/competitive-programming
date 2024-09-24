use mod_int::ModInt998244353;
use proconio::{input, marker::Usize1};

type Mint = ModInt998244353;

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
        edges: [(Usize1, Usize1); n - 1],
    };

    if n >= 1000 {
        todo!();
    }

    let mut g = vec![vec![]; n];
    for &(u, v) in &edges {
        g[u].push(v);
        g[v].push(u);
    }
    let mut ord = Vec::new();
    let mut parent = vec![0; n];
    dfs(0, usize::MAX, &g, &mut ord, &mut parent);

    let mut ans = Mint::new(0);
    for c in 0..n {
        let mut ans_c = Mint::new(0);
        // dp[i]
        // := 頂点iを根とした部分木で、
        //    頂点iを含む部分集合のうち、
        //    - iの次数が1 ⇒ i以外の全頂点の色がc, iの色は自由
        //    - それ以外 ⇒ 全頂点の色がc
        //    を満たすもの、の個数
        let mut dp = vec![Mint::new(0); n];
        for &i in ord.iter().rev() {
            let mut prod = Mint::new(1);
            let mut sum = Mint::new(0);
            for &j in &g[i] {
                if j == parent[i] {
                    continue;
                }
                prod *= dp[j] + 1;
                sum += dp[j];
            }
            if a[i] == c {
                // それぞれの子についてiに繋げる/繋げないの自由度がある
                // 頂点iの次数は気にしなくていい
                dp[i] = prod;
                ans_c += prod;
            } else {
                // 頂点iの次数が0になる、どの子とも繋げなかった場合の数を引く
                dp[i] = prod - 1;
                // 頂点iの次数が1,0になる場合の数を引く
                // 次数0のときの寄与は a[i] == c のほうで数えている
                ans_c += prod - sum - 1;
            }
        }
        ans += ans_c;
    }

    println!("{}", ans.val());
}

fn dfs(i: usize, p: usize, g: &Vec<Vec<usize>>, ord: &mut Vec<usize>, parent: &mut Vec<usize>) {
    ord.push(i);
    parent[i] = p;
    for &j in &g[i] {
        if j != p {
            dfs(j, i, g, ord, parent);
        }
    }
}
