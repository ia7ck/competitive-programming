use proconio::{input, marker::Usize1};

use crate::re_rooting_dp::re_rooting_dp;

fn main() {
    input! {
        n: usize,
        edges: [(Usize1, Usize1); n - 1],
    };

    let edges = edges
        .into_iter()
        .map(|(u, v)| (u, v, E {}))
        .collect::<Vec<_>>();

    let ans = re_rooting_dp(
        n,
        &edges,
        |i| V { d: 0, i },
        |p, ch, _e| {
            let new_d = p.d.max(ch.d + 1);
            let new_i = if p.d > ch.d + 1 {
                p.i
            } else if p.d < ch.d + 1 {
                ch.i
            } else {
                p.i.max(ch.i)
            };
            V { d: new_d, i: new_i }
        },
    );

    for ans in ans {
        println!("{}", ans.i + 1);
    }
}

struct E {}

#[derive(Clone, Debug)]
struct V {
    d: usize,
    i: usize,
}

// Bundled
#[rustfmt::skip]
#[allow(unused)]
mod re_rooting_dp {
    pub fn re_rooting_dp<E, V, F, G>(n: usize, edges: &[(usize, usize, E)], new: F, fold: G) -> Vec<V>
    where
        V: Clone,
        F: Fn(usize) -> V,
        G: Fn(&V, &V, &E) -> V,
    {
        if n == 0 {
            return Vec::new();
        }

        let (g, pre_order) = {
            let mut g = vec![vec![]; n];
            for (u, v, e) in edges {
                g[*u].push((*v, e));
                g[*v].push((*u, e));
            }
            let mut ord = Vec::with_capacity(n);
            let mut stack = vec![(0, usize::MAX)];
            while let Some((i, p)) = stack.pop() {
                ord.push(i);
                g[i].retain(|&(j, _)| j != p);
                for &(j, _) in &g[i] {
                    stack.push((j, i));
                }
            }
            (g, ord)
        };

        let dp_sub = {
            let mut dp_sub = (0..n).map(&new).collect::<Vec<_>>();
            for &i in pre_order.iter().rev() {
                for &(j, e) in &g[i] {
                    dp_sub[i] = fold(&dp_sub[i], &dp_sub[j], e);
                }
            }
            dp_sub
        };

        let mut dp_p = (0..n).map(&new).collect::<Vec<_>>();
        for i in pre_order {
            apply(dp_p[i].clone(), &g[i], &fold, &dp_sub, &mut dp_p);
        }

        dp_p.into_iter()
            .enumerate()
            .map(|(i, dp_p)| {
                g[i].iter()
                    .fold(dp_p, |acc, &(j, e)| fold(&acc, &dp_sub[j], e))
            })
            .collect::<Vec<_>>()
    }

    fn apply<E, V, G>(acc: V, children: &[(usize, &E)], fold: &G, dp_sub: &Vec<V>, dp_p: &mut Vec<V>)
    where
        V: Clone,
        G: Fn(&V, &V, &E) -> V,
    {
        if children.is_empty() {
            return;
        }

        if children.len() == 1 {
            let (j, e) = children[0];
            dp_p[j] = fold(&dp_p[j], &acc, e);
            return;
        }

        let (left, right) = children.split_at(children.len() / 2);
        let left_acc = left
            .iter()
            .fold(acc.clone(), |acc, &(j, e)| fold(&acc, &dp_sub[j], e));
        let right_acc = right
            .iter()
            .fold(acc, |acc, &(j, e)| fold(&acc, &dp_sub[j], e));
        apply(left_acc, right, fold, dp_sub, dp_p);
        apply(right_acc, left, fold, dp_sub, dp_p);
    }
}
