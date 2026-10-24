use proconio::{input, marker::Usize1};

use crate::dijkstra::{ConstEdge, dijkstra};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Usize1,
        t: Usize1,
        edges: [(Usize1, Usize1); m],
    };

    let mut adj = vec![vec![]; n];
    for (a, b) in edges {
        adj[a].push(b);
    }
    for i in 0..n {
        adj[i].sort_unstable();
    }

    let mut edges = Vec::new();
    for i in 0..n {
        if let Some((&first, rest)) = adj[i].split_first() {
            edges.push(ConstEdge::new(i, first, 0));
            for &j in rest {
                edges.push(ConstEdge::new(i, j, 1));
            }
        }
    }

    let (d, _) = dijkstra(n, &edges, s);
    if let Some(ans) = d[t] {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

// Bundled
// https://github.com/ia7ck/rust-competitive-programming/blob/bd47f3ceb41010a4c658196af9910dc80263545e/libs/dijkstra/src/lib.rs
#[rustfmt::skip]
#[allow(unused)]
mod dijkstra {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    use std::fmt::Debug;
    use std::ops::Add;

    pub trait Edge<T> {
        fn from(&self) -> usize;
        fn to(&self) -> usize;
        fn dist(&self, d: &T) -> T;
    }

    pub struct ConstEdge<T> {
        from: usize,
        to: usize,
        cost: T,
    }

    impl<T> ConstEdge<T> {
        pub fn new(from: usize, to: usize, cost: T) -> Self {
            Self { from, to, cost }
        }
    }

    impl<T> Edge<T> for ConstEdge<T>
    where
        T: Copy + Add<Output = T>,
    {
        fn from(&self) -> usize {
            self.from
        }
        fn to(&self) -> usize {
            self.to
        }
        fn dist(&self, d: &T) -> T {
            *d + self.cost
        }
    }

    pub fn dijkstra<E, T>(n: usize, edges: &[E], s: usize) -> (Vec<Option<T>>, Vec<Option<usize>>)
    where
        E: Edge<T>,
        T: Clone + Default + Ord + Debug,
    {
        let mut adj = vec![vec![]; n];
        for e in edges {
            adj[e.from()].push(e);
        }
        let mut dist = vec![Option::<T>::None; n];
        let mut heap = BinaryHeap::new();
        let mut prev = vec![None; n];
        dist[s] = Some(T::default());
        heap.push((Reverse(T::default()), s));
        while let Some((Reverse(d), v)) = heap.pop() {
            #[allow(clippy::comparison_chain)]
            match &dist[v] {
                Some(dv) => {
                    if dv < &d {
                        continue;
                    } else if dv > &d {
                        unreachable!();
                    } else {
                        assert_eq!(dv, &d);
                    }
                }
                None => unreachable!(),
            }
            for e in &adj[v] {
                let next_d = e.dist(&d);
                let to = e.to();
                match &dist[to] {
                    Some(dt) if dt <= &next_d => {
                        continue;
                    }
                    _ => {
                        dist[to] = Some(next_d.clone());
                        prev[to] = Some(v);
                        heap.push((Reverse(next_d), to));
                    }
                }
            }
        }
        (dist, prev)
    }
}
