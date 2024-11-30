use std::cmp::Reverse;

use proconio::{input, marker::Usize1};
use ac_library::{LazySegtree, MapMonoid, Max, Monoid};

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        bars: [(Usize1, Usize1, usize); n],
    };

    let bars = {
        let mut bars_ = Vec::new();
        for i in 0..n {
            let (r, c, l) = bars[i];
            bars_.push((r, c, l, i));
        }
        bars_.sort_by_key(|&(r, _, _, _)| Reverse(r));
        bars_
    };

    let mut seg = LazySegtree::<MaxUpdate>::new(w);
    let mut ans = vec![0; n];
    for (_, c, l, i) in bars {
        let m = seg.prod(c..(c + l));
        ans[i] = h - m;
        seg.apply_range(c..(c + l), Some(m + 1));
    }

    for ans in ans {
        println!("{}", ans);
    }
}

struct MaxUpdate;
impl MapMonoid for MaxUpdate {
    type M = Max<usize>;
    type F = Option<usize>;
    
    fn identity_map() -> Self::F {
        None
    }
    
    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        if let Some(f) = f {
            *f
        } else {
            *x
        }
    }
    
    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        if let Some(f) = f {
            Some(*f)
        } else {
            *g
        }
    }
}
