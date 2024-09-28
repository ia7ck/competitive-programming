use std::{collections::HashMap, mem};

use mod_int::ModInt998244353;
use proconio::{input, marker::Usize1};

type Mint = ModInt998244353;

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
        edges: [(Usize1, Usize1); n - 1],
    };

    let mut g = vec![vec![]; n];
    for &(u, v) in &edges {
        g[u].push(v);
        g[v].push(u);
    }
    let mut ord = Vec::new();
    let mut parent = vec![0; n];
    dfs(0, usize::MAX, &g, &mut ord, &mut parent);

    let mut ans = Mint::new(0);
    if false {
        // N^2
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
    } else {
        let h = auxiliary_tree(&g, &ord, &parent, &a);
        let mut dp = vec![HashMap::<usize, Mint>::new(); n];
        for &i in ord.iter().rev() {
            for (&c, children) in &h[i] {
                let mut prod = Mint::new(1);
                let mut sum = Mint::new(0);
                for &j in children {
                    prod *= dp[j][&c] + 1;
                    sum += dp[j][&c];
                }
                if a[i] == c {
                    dp[i].insert(c, prod);
                    ans += prod;
                } else {
                    dp[i].insert(c, prod - 1);
                    ans += prod - sum - 1;
                }
            }
        }
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

fn auxiliary_tree(
    g: &Vec<Vec<usize>>,
    ord: &Vec<usize>,
    parent: &Vec<usize>,
    color: &Vec<usize>,
) -> Vec<HashMap<usize, Vec<usize>>> {
    // lca[x][c]: 部分木x内の色cの頂点全体にわたるLCA
    let mut lca = vec![HashMap::new(); g.len()];
    // h[x][c]: 色cに関して圧縮した木における頂点xの子たち
    let mut h = vec![HashMap::<_, Vec<_>>::new(); g.len()];
    for &x in ord.iter().rev() {
        // 気持ち的には、ls[c]: 頂点xの子yについてlca[y][c]を集めたもの、だけど
        // マージテクのためにlca[x]と同じ型にしている
        let mut ls = HashMap::new();
        // lsからあふれた頂点たち
        let mut rest = HashMap::<_, Vec<_>>::new();
        for &y in &g[x] {
            if parent[x] == y {
                continue;
            }
            // マージテク
            if ls.len() < lca[y].len() {
                mem::swap(&mut ls, &mut lca[y]);
            }
            for (&c, &z) in lca[y].iter() {
                if let Some(old) = ls.insert(c, z) {
                    rest.entry(c).or_default().push(old);
                }
            }
        }

        for (&c, rest) in &rest {
            // 部分木x内に色cの頂点が2個以上あるので、圧縮後の木に頂点xが残る
            let e = h[x].entry(c).or_default();
            assert!(ls.contains_key(&c));
            e.push(ls[&c]);
            e.extend(rest);
        }

        if !rest.contains_key(&color[x]) {
            let ch = match ls.get(&color[x]) {
                Some(&y) => vec![y],
                // 圧縮後の木で頂点xが葉になるケース
                None => vec![],
            };
            let old = h[x].insert(color[x], ch);
            assert!(old.is_none());
        }

        // ちょうど1回現われる色のLCAはそのまま
        lca[x] = ls;
        for (c, _) in rest {
            // 2回以上現われる色cの頂点たちのLCAをxに更新
            lca[x].insert(c, x);
        }
        // 色color[x]は登場回数に関係なくLCAをxに更新
        lca[x].insert(color[x], x);
    }
    h
}
