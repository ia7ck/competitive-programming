use std::cmp::Reverse;

use proconio::{input, marker::Usize1};

use crate::arg_cmp::ArgCmp;

fn main() {
    input! {
        n: usize,
        q: usize,
        xy: [(i64, i64); n],
        ab: [(Usize1, Usize1); q],
    };

    let clock_wise = |(x, y): (i64, i64)| Reverse(ArgCmp::new(x, y));

    let mut xy_sorted = xy.clone();
    xy_sorted.sort_by_key(|&p| clock_wise(p));

    for (a, b) in ab {
        let p_a = xy[a];
        let p_b = xy[b];
        let i_a = xy_sorted.partition_point(|&p| clock_wise(p) < clock_wise(p_a));
        let i_b = xy_sorted.partition_point(|&p| clock_wise(p) <= clock_wise(p_b));
        let ans = if i_b > i_a {
            i_b - i_a
        } else {
            n - (i_a - i_b)
        };

        println!("{}", ans);
    }
}

// Bundled
#[rustfmt::skip]
#[allow(unused)]
mod arg_cmp {
    use ::std::cmp::Ordering;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct ArgCmp((i64, i64));

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    pub enum Quadrant {
        First,
        Second,
        Third,
        Fourth,
    }

    impl ArgCmp {
        pub fn new(x: i64, y: i64) -> Self {
            assert_ne!((x, y), (0, 0));
            Self((x, y))
        }

        pub fn x(&self) -> i64 {
            self.0.0
        }

        pub fn y(&self) -> i64 {
            self.0.1
        }

        fn is_lower_half(&self) -> bool {
            self.y() < 0 || (self.y() == 0 && self.x() < 0)
        }

        pub fn quadrant(&self) -> Quadrant {
            match (self.is_lower_half(), self.x().cmp(&0)) {
                (false, Ordering::Greater) => Quadrant::First,
                (false, _) => Quadrant::Second,
                (true, Ordering::Less) => Quadrant::Third,
                (true, _) => Quadrant::Fourth,
            }
        }
    }

    impl Ord for ArgCmp {
        fn cmp(&self, other: &Self) -> Ordering {
            self.is_lower_half()
                .cmp(&other.is_lower_half())
                .then_with(||
                (self.y() * other.x()).cmp(&(self.x() * other.y())))
        }
    }

    impl PartialOrd for ArgCmp {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
}
