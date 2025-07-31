use re_rooting_dp::re_rooting_dp;
use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let input = io::read_to_string(io::stdin())?;
    let mut tokens = input.split_whitespace();

    let n = tokens.next().unwrap().parse::<usize>()?;
    let mut edges = Vec::with_capacity(n - 1);
    for _ in 0..(n - 1) {
        let a = tokens.next().unwrap().parse::<usize>()?;
        let b = tokens.next().unwrap().parse::<usize>()?;
        let c = tokens.next().unwrap().parse::<u64>()?;
        edges.push((a, b, E(c)));
    }

    let ans = re_rooting_dp(
        n,
        &edges,
        |i| V {
            d: 0,
            self_: i,
            to: i,
            farthest: i,
        },
        |p, ch, e| {
            let d_ch = ch.d + e.0;
            if p.d > d_ch {
                p.clone()
            } else {
                V {
                    d: d_ch,
                    self_: p.self_,
                    to: ch.self_,
                    farthest: ch.farthest,
                }
            }
        },
    );

    let d_max = ans.iter().map(|v| v.d).max().unwrap();
    let u = ans.iter().find(|v| v.d == d_max).cloned().unwrap();
    let v = ans
        .iter()
        .find(|v| v.d == d_max && v.farthest == u.self_)
        .cloned()
        .unwrap();
    let mut path_u = std::iter::successors(Some(u), |u| {
        let next_u = ans[u.to].clone();
        if u.farthest == next_u.farthest {
            Some(next_u)
        } else {
            None
        }
    })
    .collect::<Vec<_>>();
    let mut path_v = std::iter::successors(Some(v), |v| {
        let next_v = ans[v.to].clone();
        if v.farthest == next_v.farthest {
            Some(next_v)
        } else {
            None
        }
    })
    .collect::<Vec<_>>();

    path_v.reverse();
    path_u.append(&mut path_v);

    println!("{} {}", d_max, path_u.len());
    println!(
        "{}",
        path_u
            .iter()
            .map(|u| u.self_.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );

    Ok(())
}

struct E(u64);

#[derive(Debug, Clone)]
struct V {
    d: u64,
    self_: usize,
    to: usize,
    farthest: usize,
}

#[allow(unused)]
mod re_rooting_dp {
    pub fn re_rooting_dp<E, V, F, G>(
        n: usize,
        edges: &[(usize, usize, E)],
        new: F,
        fold: G,
    ) -> Vec<V>
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

    fn apply<E, V, G>(
        acc: V,
        children: &[(usize, &E)],
        fold: &G,
        dp_sub: &Vec<V>,
        dp_p: &mut Vec<V>,
    ) where
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
