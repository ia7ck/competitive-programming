use std::collections::VecDeque;

use memoise::memoise;
use proconio::{input, marker::Usize1};
use union_find::UnionFind;

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, usize); m],
    };

    let mut uf = UnionFind::new(n);
    for &(a, b, _) in &abc {
        uf.unite(a, b);
    }
    let mut root = vec![0; n];
    let mut components = vec![vec![]; n];
    for i in 0..n {
        let r = uf.find(i);
        root[i] = r;
        components[r].push(i);
    }

    let mut g = vec![vec![]; n];
    let mut rev_g = vec![vec![]; n];
    for &(a, b, c) in &abc {
        // rank[a] - rank[b] = c
        g[b].push((a, c));
        rev_g[a].push((b, c));
    }

    let mut ans = Vec::new();
    for i in 0..n {
        let mut candidate = Vec::new();
        for r in 0..n {
            if let Some(ranks) = fit_ranks(n, i, r, &root, &components, &g, &rev_g) {
                let ranks = {
                    let mut ranks_ = 0_usize;
                    for r in ranks {
                        ranks_ ^= 1 << r;
                    }
                    ranks_
                };
                if solve(n, i, &root, 0, &components, &g, &rev_g, ranks) {
                    candidate.push(r);
                }
            }
        }
        assert!(candidate.len() >= 1);
        if candidate.len() == 1 {
            ans.push((candidate[0] + 1).to_string());
        } else {
            ans.push("-1".to_string());
        }
    }

    for i in 0..n {
        print!("{}", ans[i]);
        if i + 1 < n {
            print!(" ");
        } else {
            print!("\n");
        }
    }
}

#[memoise(i_c <= 16, rank_bit < 65536)]
fn solve(
    n: usize,
    skip: usize,
    root: &Vec<usize>,
    i_c: usize,
    components: &Vec<Vec<usize>>,
    g: &Vec<Vec<(usize, usize)>>,
    rev_g: &Vec<Vec<(usize, usize)>>,
    rank_bit: usize,
) -> bool {
    if i_c == components.len() {
        return true;
    }
    if root[skip] == i_c || components[i_c].is_empty() {
        return solve(n, skip, root, i_c + 1, components, g, rev_g, rank_bit);
    }
    let s = components[i_c][0];
    for r in 0..n {
        if let Some(ranks) = fit_ranks(n, s, r, root, components, &g, &rev_g) {
            let mut ok = true;
            let mut new_rank_bit = rank_bit;
            for &r in &ranks {
                if new_rank_bit >> r & 1 == 1 {
                    ok = false;
                    break;
                }
                new_rank_bit ^= 1 << r;
            }
            if ok && solve(n, skip, root, i_c + 1, components, g, rev_g, new_rank_bit) {
                return true;
            }
        }
    }
    false
}

fn fit_ranks(
    n: usize,
    i: usize,
    r: usize,
    root: &Vec<usize>,
    components: &Vec<Vec<usize>>,
    g: &Vec<Vec<(usize, usize)>>,
    rev_g: &Vec<Vec<(usize, usize)>>,
) -> Option<Vec<usize>> {
    let mut ranks = Vec::new();
    let mut seen = vec![false; n];
    let mut que = VecDeque::new();
    seen[i] = true;
    que.push_back((i, r));
    while let Some((i, r)) = que.pop_front() {
        ranks.push(r);
        for &(j, c) in &g[i] {
            // rank[j] - rank[i] = c
            if r + c < n {
                if seen[j] == false {
                    seen[j] = true;
                    que.push_back((j, r + c));
                }
            } else {
                return None;
            }
        }
        for &(j, c) in &rev_g[i] {
            // rank[i] - rank[j] = c
            if r >= c {
                if seen[j] == false {
                    seen[j] = true;
                    que.push_back((j, r - c));
                }
            } else {
                return None;
            }
        }
    }
    ranks.sort();
    ranks.dedup();
    assert_eq!(ranks.len(), components[root[i]].len());
    return Some(ranks);
}
