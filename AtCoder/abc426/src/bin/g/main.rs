use std::ops::Range;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        wv: [(usize, u64); n],
        q: usize,
        queries: [(Usize1, Usize1, usize); q],
    };

    let queries = queries
        .into_iter()
        .enumerate()
        .map(|(i, (l, r, c))| Query {
            index: i,
            l,
            r: r + 1,
            c,
        })
        .collect::<Vec<_>>();

    let mut ans = vec![0; q];
    solve(0..n, queries, &wv, &mut ans);

    for ans in ans {
        println!("{}", ans);
    }
}

macro_rules! chmax {
    ($a: expr, $b: expr) => {
        $a = $a.max($b);
    };
}

fn solve(range: Range<usize>, queries: Vec<Query>, wv: &Vec<(usize, u64)>, ans: &mut Vec<u64>) {
    debug_assert!(queries
        .iter()
        .all(|q| range.start <= q.l && q.r <= range.end));

    if range.start + 1 == range.end {
        for q in queries {
            let (w, v) = wv[q.l];
            if w <= q.c {
                ans[q.index] = v;
            }
        }
        return;
    }

    let m = (range.start + range.end) / 2;
    // dp_l[i][j] := [m - i, m), sum(w) <= j
    let mut dp_l = vec![vec![None; 501]; m - range.start + 1];
    // dp_r[i][j] := [m, m + i), sum(w) <= j
    let mut dp_r = vec![vec![None; 501]; range.end - m + 1];
    dp_l[0][0] = Some(0);
    for (i, &(w, v)) in wv[range.start..m].iter().rev().enumerate() {
        for j in 0..=500 {
            chmax!(dp_l[i + 1][j], dp_l[i][j]);
            if let Some(s) = dp_l[i][j] {
                if j + w <= 500 {
                    chmax!(dp_l[i + 1][j + w], Some(s + v));
                }
            }
        }

        for j in 0..500 {
            chmax!(dp_l[i + 1][j + 1], dp_l[i + 1][j]);
        }
    }
    dp_r[0][0] = Some(0);
    for (i, &(w, v)) in wv[m..range.end].iter().enumerate() {
        for j in 0..=500 {
            chmax!(dp_r[i + 1][j], dp_r[i][j]);
            if let Some(s) = dp_r[i][j] {
                if j + w <= 500 {
                    chmax!(dp_r[i + 1][j + w], Some(s + v));
                }
            }
        }

        for j in 0..500 {
            chmax!(dp_r[i + 1][j + 1], dp_r[i + 1][j]);
        }
    }

    let mut l_queries = Vec::new();
    let mut r_queries = Vec::new();
    for q in queries {
        if q.r <= m {
            l_queries.push(q);
        } else if m <= q.l {
            r_queries.push(q);
        } else {
            assert!(q.l < m && m < q.r);
            for j in 0..=q.c {
                let sl = dp_l[m - q.l][j].unwrap_or(0);
                let sr = dp_r[q.r - m][q.c - j].unwrap_or(0);
                chmax!(ans[q.index], sl + sr);
            }
        }
    }

    solve(range.start..m, l_queries, wv, ans);
    solve(m..range.end, r_queries, wv, ans);
}

#[derive(Debug)]
struct Query {
    index: usize,
    // [l, r)
    l: usize,
    r: usize,
    c: usize,
}
