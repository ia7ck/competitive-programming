use ac_library::{LazySegtree, MapMonoid, Max};
use proconio::input;

fn main() {
    input! {
        n: usize,
        lr: [(usize, usize); n],
        q: usize,
        xs: [usize; q],
    };

    let mut seg = LazySegtree::<MaxAdd>::new(500_000 + 1);
    for i in 0..=500_000 {
        seg.set(i, i);
    }
    for (l, r) in lr {
        let p = seg.max_right(0, |x| x < l);
        let q = seg.max_right(0, |x| x <= r);
        seg.apply_range(p..q, 1);
    }

    for x in xs {
        let ans = seg.get(x);
        println!("{}", ans);
    }
}

struct MaxAdd;
impl MapMonoid for MaxAdd {
    type M = Max<usize>;
    type F = usize;

    fn identity_map() -> Self::F {
        0
    }

    fn mapping(f: &Self::F, x: &usize) -> usize {
        *f + *x
    }

    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        *f + *g
    }
}
