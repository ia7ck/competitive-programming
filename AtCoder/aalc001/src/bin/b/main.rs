use ac_library::{LazySegtree, MapMonoid, Min};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u64; n],
    };

    let mut seg = LazySegtree::<F>::new(n);
    for i in 0..n {
        seg.set(i, a[i]);
    }

    for _ in 0..q {
        input! {
            op: u8,
        };

        if op == 1 {
            input! {
                l: Usize1,
                r: Usize1,
                x: u64,
            };

            seg.apply_range(l..=r, x);
        } else {
            input! {
                l: Usize1,
                r: Usize1,
            };

            let ans = seg.prod(l..=r);
            println!("{}", ans);
        }
    }
}

struct F;
impl MapMonoid for F {
    type M = Min<u64>;

    type F = u64;

    fn identity_map() -> Self::F {
        0
    }

    fn mapping(
        f: &Self::F,
        x: &<Self::M as ac_library::Monoid>::S,
    ) -> <Self::M as ac_library::Monoid>::S {
        *f + *x
    }

    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        *f + *g
    }
}
