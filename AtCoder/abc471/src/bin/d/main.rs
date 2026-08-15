use ac_library::{LazySegtree, MapMonoid, Monoid};
use proconio::input;

fn main() {
    input! {
        q: usize,
        v: i64,
    };

    let mut last_t = Option::<i64>::None;
    let mut seg = LazySegtree::<MM>::new(q);
    for i in 0..q {
        input! {
            op: u8,
            t: i64,
        };

        if let Some(last_t) = last_t {
            seg.apply_range(..i, t - last_t);
        }

        if op == 1 {
            input! {
                w: i64,
            };

            seg.set(i, (w, i));
        } else {
            let (ans, p) = seg.prod(..i);
            if ans < 0 {
                println!("-1");
            } else {
                println!("{}", ans.min(v));
                seg.set(p, (i64::MIN, p));
            }
        }

        last_t = Some(t);
    }
}

struct M {}

impl Monoid for M {
    type S = (i64, usize);

    fn identity() -> Self::S {
        (i64::MIN, usize::MAX)
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        if a.0 > b.0 { *a } else { *b }
    }
}

struct MM {}

impl MapMonoid for MM {
    type M = M;

    type F = i64;

    fn identity_map() -> Self::F {
        0
    }

    fn mapping(
        f: &Self::F,
        x: &<Self::M as ac_library::Monoid>::S,
    ) -> <Self::M as ac_library::Monoid>::S {
        (f + x.0, x.1)
    }

    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        f + g
    }
}
