use std::collections::BinaryHeap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [u32; n],
        q: usize,
    };

    #[derive(Debug, Clone, Copy)]
    enum Query {
        ChMax(usize, usize, u32),
        Cancel(usize),
        Get(usize),
    }

    let mut queries = Vec::new();
    for _ in 0..q {
        input! {
            op: u8,
        };
        if op == 1 {
            input! {
                l: Usize1,
                r: Usize1,
                x: u32,
            };
            queries.push(Query::ChMax(l, r, x));
        } else if op == 2 {
            input! {
                i: Usize1,
            };
            queries.push(Query::Cancel(i));
        } else {
            input! {
                i: Usize1,
            };
            queries.push(Query::Get(i));
        }
    }

    let mut seg = DualSegmentTree::new(n, ErasableBinaryHeap::new());
    for i in 0..n {
        seg.apply(i..(i + 1), |heap| heap.push(a[i]));
    }

    for &q in &queries {
        match q {
            Query::ChMax(l, r, x) => {
                seg.apply(l..(r + 1), |heap| heap.push(x));
            }
            Query::Cancel(i) => {
                let (l, r, x) = match queries[i] {
                    Query::ChMax(l, r, x) => (l, r, x),
                    _ => unreachable!(),
                };
                seg.apply(l..(r + 1), |heap| heap.erase(x));
            }
            Query::Get(i) => {
                let x = seg.reduce(i, 0_u32, |acc, heap| {
                    let top = heap.peek().unwrap_or(&0);
                    *acc.max(top)
                });
                println!("{}", x);
            }
        }
    }
}

use std::ops;

#[derive(Debug, Clone)]
struct ErasableBinaryHeap {
    live: BinaryHeap<u32>,
    dead: BinaryHeap<u32>,
}

impl ErasableBinaryHeap {
    fn new() -> Self {
        Self {
            live: BinaryHeap::new(),
            dead: BinaryHeap::new(),
        }
    }

    fn push(&mut self, x: u32) {
        self.live.push(x);
    }

    fn erase(&mut self, x: u32) {
        self.dead.push(x);
    }

    fn peek(&mut self) -> Option<&u32> {
        while let Some(x) = self.dead.peek() {
            if self.live.peek() == Some(x) {
                self.live.pop();
                self.dead.pop();
            } else {
                break;
            }
        }
        self.live.peek()
    }
}

struct DualSegmentTree<F> {
    n: usize,
    mappings: Vec<F>,
}

impl<F> DualSegmentTree<F>
where
    F: Clone,
{
    fn new(n: usize, identity: F) -> Self {
        let n = n.next_power_of_two();
        let mappings = vec![identity.clone(); n * 2 - 1];
        Self { n, mappings }
    }

    // ううう
    fn reduce<T, G>(&mut self, i: usize, e: T, g: G) -> T
    where
        G: Fn(&T, &mut F) -> T,
    {
        let mut i = i + self.n - 1;
        let mut acc = e;
        while i > 0 {
            acc = g(&acc, &mut self.mappings[i]);
            i = (i - 1) / 2;
        }
        g(&acc, &mut self.mappings[0])
    }

    fn apply<U>(&mut self, range: ops::Range<usize>, update: U)
    where
        U: Fn(&mut F),
    {
        self._apply(&range, &update, 0, 0..self.n);
    }

    fn _apply<U>(
        &mut self,
        range: &ops::Range<usize>,
        update: &U,
        i: usize,
        i_range: ops::Range<usize>,
    ) where
        U: Fn(&mut F),
    {
        if range.end <= i_range.start || i_range.end <= range.start {
            return;
        }
        if range.start <= i_range.start && i_range.end <= range.end {
            update(&mut self.mappings[i]);
            return;
        }
        let m = (i_range.start + i_range.end) / 2;
        self._apply(range, update, i * 2 + 1, i_range.start..m);
        self._apply(range, update, i * 2 + 2, m..i_range.end);
    }
}
