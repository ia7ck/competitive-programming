use ac_library::{LazySegtree, MapMonoid, Monoid};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        q: usize,
        queries: [(Usize1, Usize1, i64); q],
    };

    let mut seg = LazySegtree::<F>::new(n);
    for i in 0..n {
        seg.set(
            i,
            S {
                sum: a[i],
                min: a[i],
                min_index: i,
                len: 1,
            },
        );
    }

    for (l, r, k) in queries {
        let before = seg.prod(l..=r);

        seg.apply_range(l..=r, F(-k));
        loop {
            let x = seg.prod(l..=r);
            if x.min < 0 {
                seg.set(x.min_index, M::identity());
            } else {
                break;
            }
        }

        let after = seg.prod(l..=r);
        println!("{}", before.sum - after.sum);
    }
}

struct M;

#[derive(Clone)]
struct S {
    sum: i64,
    min: i64,
    min_index: usize,
    len: usize,
}

impl Monoid for M {
    type S = S;

    fn identity() -> Self::S {
        S {
            sum: 0,
            min: i64::MAX,
            min_index: usize::MAX,
            len: 0,
        }
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        S {
            sum: a.sum + b.sum,
            min: a.min.min(b.min),
            min_index: if a.min <= b.min {
                a.min_index
            } else {
                b.min_index
            },
            len: a.len + b.len,
        }
    }
}

#[derive(Clone)]
struct F(i64);

impl MapMonoid for F {
    type M = M;

    type F = F;

    fn identity_map() -> Self::F {
        F(0)
    }

    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        S {
            sum: x.sum + f.0 * x.len as i64,
            min: x.min + f.0,
            min_index: x.min_index,
            len: x.len,
        }
    }

    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        F(f.0 + g.0)
    }
}
