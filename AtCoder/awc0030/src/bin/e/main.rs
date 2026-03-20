use std::collections::HashMap;

use proconio::input;

use crate::divisors::Divisors;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut counter = HashMap::new();
    for a in a {
        *counter.entry(a).or_insert(0) += 1;
    }

    let mut c = vec![0_usize; 1_000_000 + 1];
    for (a, x) in counter {
        for d in a.divisors() {
            c[d] += x;
        }
    }

    let mut ans = 0;
    for d in 1..c.len() {
        ans = ans.max(d * c[d]);
    }

    println!("{}", ans);
}

// Bundled
#[rustfmt::skip]
#[allow(unused)]
mod divisors {
    pub trait Divisors: Sized {
        fn divisors(self) -> Vec<Self>;
    }

    macro_rules! impl_divisors {
        ($($t:ty),+) => {
            $(
                impl Divisors for $t {
                    fn divisors(self) -> Vec<Self> {
                        let mut res = vec![];
                        let mut large = vec![];
                        for k in ((1 as Self)..).take_while(|&k| k.saturating_mul(k) <= self) {
                            if self % k == 0 {
                                res.push(k);
                                if self / k != k {
                                    large.push(self / k);
                                }
                            }
                        }
                        large.reverse();
                        res.append(&mut large);
                        res
                    }
                }
            )+
        };
    }

    impl_divisors!(usize, u32, u64);
}
