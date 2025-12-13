use proconio::{input, marker::Chars};

use crate::{
    dijkstra::{ConstEdge, dijkstra},
    grid_search::around,
};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };

    let id = |i: usize, j: usize| i * w + j;
    let mut edges = Vec::new();
    for i in 0..h {
        for j in 0..w {
            for (ni, nj) in around(i, j).y_range(0..h).x_range(0..w).directions(&[
                (-1, 0),
                (0, -1),
                (1, 0),
                (0, 1),
            ]) {
                if s[i][j] != '#' && s[ni][nj] != '#' {
                    edges.push(ConstEdge::new(id(i, j), id(ni, nj), 2));
                    edges.push(ConstEdge::new(id(ni, nj), id(i, j), 2));
                }
            }
        }
    }
    for i in 0..h {
        for j in 0..w {
            if 'a' <= s[i][j] && s[i][j] <= 'z' {
                let c = s[i][j] as usize - 'a' as usize;
                edges.push(ConstEdge::new(id(i, j), h * w + c, 1));
                edges.push(ConstEdge::new(h * w + c, id(i, j), 1));
            }
        }
    }

    let (d, _) = dijkstra(h * w + 26, &edges, id(0, 0));
    if let Some(ans) = d[id(h - 1, w - 1)] {
        assert_eq!(ans % 2, 0);
        println!("{}", ans / 2);
    } else {
        println!("-1");
    }
}

// Bundled
#[rustfmt::skip]
#[allow(unused)]
mod grid_search {
    use std::ops::Range;

    pub struct Around<'a> {
        y: usize,
        x: usize,
        y_range: Range<usize>,
        x_range: Range<usize>,
        directions: &'a [(isize, isize)],
    }

    pub fn around<'a>(y: usize, x: usize) -> Around<'a> {
        Around {
            y,
            x,
            y_range: 0..usize::MAX,
            x_range: 0..usize::MAX,
            directions: &[],
        }
    }

    impl<'a> Around<'a> {
        pub fn y_range(self, r: Range<usize>) -> Self {
            Self { y_range: r, ..self }
        }
        pub fn x_range(self, r: Range<usize>) -> Self {
            Self { x_range: r, ..self }
        }
        pub fn directions(self, dirs: &'a [(isize, isize)]) -> Self {
            Self {
                directions: dirs,
                ..self
            }
        }
    }

    impl<'a> Iterator for Around<'a> {
        type Item = (usize, usize);
        fn next(&mut self) -> Option<Self::Item> {
            while let Some((&(dy, dx), rest)) = self.directions.split_first() {
                self.directions = rest;
                match (self.y.checked_add_signed(dy), self.x.checked_add_signed(dx)) {
                    (Some(ny), Some(nx))
                        if self.y_range.contains(&self.y)
                            && self.x_range.contains(&self.x)
                            && self.y_range.contains(&ny)
                            && self.x_range.contains(&nx) =>
                    {
                        return Some((ny, nx));
                    }
                    _ => {}
                }
            }
            None
        }
    }
}

// Bundled
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
