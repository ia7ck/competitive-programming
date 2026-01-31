use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        t: u64,
        a: [u64; n],
    };

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Event {
        Aoki(u64),
        Open(u64),
    }

    impl Event {
        fn inner(&self) -> u64 {
            match self {
                Event::Aoki(x) => *x,
                Event::Open(x) => *x,
            }
        }
    }

    impl Ord for Event {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.inner().cmp(&other.inner())
        }
    }

    impl PartialOrd for Event {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut heap = BinaryHeap::new();
    for a in a {
        heap.push(Reverse(Event::Aoki(a)));
    }

    let mut ans = 0;
    let mut open = true;
    let mut last = 0;
    while let Some(Reverse(e)) = heap.pop() {
        match e {
            Event::Aoki(x) => {
                if open {
                    open = false;
                    ans += x - last;
                    heap.push(Reverse(Event::Open(x + 100)));
                } else {
                    //
                }
                last = x;
            }
            Event::Open(x) => {
                if open {
                    unreachable!();
                } else {
                    open = true;
                }
                last = x;
            }
        }
    }

    if t >= last {
        ans += t - last;
    }

    println!("{}", ans);
}
