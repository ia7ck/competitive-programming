use std::cmp;

use ac_library::{LazySegtree, MapMonoid, Monoid};
use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        w: usize,
    };

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Event {
        In((usize, usize)),
        Out((usize, usize)),
    }

    impl Ord for Event {
        fn cmp(&self, other: &Self) -> cmp::Ordering {
            let (t1, x1) = match self {
                Event::In((t, x)) => (t, x),
                Event::Out((t, x)) => (t, x),
            };
            let (t2, x2) = match other {
                Event::In((t, x)) => (t, x),
                Event::Out((t, x)) => (t, x),
            };
            // out < in
            (t1, !matches!(self, Event::Out(_)), x1).cmp(&(t2, !matches!(other, Event::Out(_)), x2))
        }
    }

    impl PartialOrd for Event {
        fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut events = Vec::new();
    for _ in 0..n {
        input! {
            t: usize,
            x: usize,
        };
        events.push(Event::In((t, x)));
        events.push(Event::Out((t + d, x)));
    }

    events.sort();
    let mut seg = LazySegtree::<F>::from(vec![0; 400_000 + 1]);
    let mut ans = i64::MIN;
    for event in events {
        match event {
            Event::In((_, x)) => {
                seg.apply_range(x..(x + w), 1);
            }
            Event::Out((_, x)) => {
                seg.apply_range(x..(x + w), -1);
            }
        }
        ans = ans.max(seg.all_prod());
    }
    println!("{}", ans);
}

// max
struct M;
impl Monoid for M {
    type S = i64;

    fn identity() -> Self::S {
        i64::MIN / 2
    }

    fn binary_operation(&a: &Self::S, &b: &Self::S) -> Self::S {
        a.max(b)
    }
}

// add
struct F;
impl MapMonoid for F {
    type M = M;
    type F = i64;

    fn identity_map() -> Self::F {
        0
    }

    fn mapping(&f: &Self::F, &x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        f + x
    }

    fn composition(&f: &Self::F, &g: &Self::F) -> Self::F {
        f + g
    }
}
