use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};
use union_find::UnionFind;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut points: [(u32, u32); n],
    };

    let mut uf = UnionFind::new(n + q);
    let mut heap = BinaryHeap::new();
    for i in 0..n {
        for j in (i + 1)..n {
            heap.push((Reverse(manhattan(points[i], points[j])), (i, j)));
        }
    }

    for _ in 0..q {
        input! {
            op: u8,
        };

        if op == 1 {
            input! {
                a: u32,
                b: u32,
            };
            let j = points.len();
            for (i, &(x, y)) in points.iter().enumerate() {
                heap.push((Reverse(manhattan((a, b), (x, y))), (i, j)));
            }
            points.push((a, b));
        } else if op == 2 {
            let ans = solve(&mut heap, &mut uf);
            match ans {
                Some(ans) => println!("{}", ans),
                None => println!("-1"),
            }
        } else {
            input! {
                u: Usize1,
                v: Usize1,
            };
            if uf.same(u, v) {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}

fn manhattan((x1, y1): (u32, u32), (x2, y2): (u32, u32)) -> u32 {
    x1.abs_diff(x2) + y1.abs_diff(y2)
}

type Heap = BinaryHeap<(Reverse<u32>, (usize, usize))>;

fn solve(heap: &mut Heap, uf: &mut UnionFind) -> Option<u32> {
    let d = loop {
        let &(Reverse(d), (i, j)) = heap.peek()?;
        if uf.same(i, j) {
            heap.pop();
        } else {
            break d;
        }
    };
    let mut edges = Vec::new();
    while let Some(&(Reverse(d2), (i, j))) = heap.peek() {
        if d == d2 {
            edges.push((i, j));
            heap.pop();
        } else {
            break;
        }
    }
    for (u, v) in edges {
        uf.unite(u, v);
    }
    Some(d)
}
