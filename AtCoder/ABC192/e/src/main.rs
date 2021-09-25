use proconio::input;

#[derive(Clone, Copy)]
struct EdgeImpl {
    from: usize,
    to: usize,
    t: u64,
    k: u64,
}

impl Edge<u64> for EdgeImpl {
    fn from(&self) -> usize {
        self.from
    }
    fn to(&self) -> usize {
        self.to
    }
    fn dist(&self, d: u64) -> u64 {
        (d + self.k - 1) / self.k * self.k + self.t
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        start: usize,
        goal: usize,
    }
    let start = start - 1;
    let goal = goal - 1;
    let mut edges = Vec::new();
    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
            t: u64,
            k: u64,
        }
        edges.push(EdgeImpl {
            from: a - 1,
            to: b - 1,
            t,
            k,
        });
        edges.push(EdgeImpl {
            from: b - 1,
            to: a - 1,
            t,
            k,
        });
    }
    let (d, _prev) = dijkstra(n, edges.iter().copied(), start);
    if let Some(ans) = d[goal] {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Debug;
use std::ops::Add;

pub trait Edge<T> {
    fn from(&self) -> usize;
    fn to(&self) -> usize;
    fn dist(&self, d: T) -> T;
}

pub fn dijkstra<I, E, T>(n: usize, edges: I, s: usize) -> (Vec<Option<T>>, Vec<Option<usize>>)
where
    I: Iterator<Item = E>,
    E: Edge<T> + Clone,
    T: Copy + Add<Output = T> + Default + Ord + Debug,
{
    let mut adj = vec![vec![]; n];
    for e in edges {
        adj[e.from()].push(e);
    }
    let mut dist = vec![None; n];
    let mut heap = BinaryHeap::new();
    let mut prev = vec![None; n];
    dist[s] = Some(T::default());
    heap.push((Reverse(T::default()), s));
    while let Some((Reverse(d), v)) = heap.pop() {
        match dist[v] {
            Some(dv) => {
                if dv < d {
                    continue;
                } else {
                    assert_eq!(dv, d);
                }
            }
            None => unreachable!(),
        }
        for e in &adj[v] {
            let next_d = e.dist(d);
            let to = e.to();
            match dist[to] {
                Some(dt) if dt <= next_d => {
                    continue;
                }
                _ => {
                    dist[to] = Some(next_d);
                    prev[to] = Some(v);
                    heap.push((Reverse(next_d), to));
                }
            }
        }
    }
    (dist, prev)
}
