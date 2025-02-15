use std::cmp::Reverse;

use proconio::input;
use segment_tree::SegmentTree;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u32; n],
        rx: [(usize, u32); q],
    };

    #[derive(Debug)]
    enum Event {
        A(usize, u32),
        Query(usize, (usize, u32)),
    }
    let mut events = Vec::new();
    for i in 0..n {
        events.push(Event::A(i, a[i]));
    }
    for i in 0..q {
        events.push(Event::Query(i, rx[i]));
    }
    events.sort_by_key(|e| match e {
        Event::A(i, x) => (*x, Reverse(*i), 0),
        Event::Query(_, (_, x)) => (
            *x,
            Reverse(0), // no matter
            1,
        ),
    });
    let mut seg = SegmentTree::new(n, 0, |x, y| *x.max(y));
    let mut ans = vec![0; q];
    for e in events {
        match e {
            Event::A(i, _) => {
                let m = seg.fold(0..i);
                seg.set(i, m + 1);
            }
            Event::Query(i, (r, _)) => {
                ans[i] = seg.fold(0..r);
            }
        }
    }
    for ans in ans {
        println!("{}", ans);
    }
}
