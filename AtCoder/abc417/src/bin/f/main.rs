use ac_library::LazySegtree;
use ac_library::MapMonoid;
use ac_library::ModInt998244353;
use ac_library::Monoid;
use proconio::input;
use proconio::marker::Usize1;

type Mint = ModInt998244353;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [u32; n],
        lr: [(Usize1, Usize1); m],
    };

    let mut seg = LazySegtree::<F>::new(n);
    for i in 0..n {
        seg.set(
            i,
            S {
                size: 1,
                value: Mint::new(a[i]),
            },
        );
    }

    for (l, r) in lr {
        let sum = seg.prod(l..=r);
        seg.apply_range(l..=r, F(Some(sum.value / (r - l + 1))));
    }

    let ans = (0..n)
        .map(|i| seg.get(i).value.to_string())
        .collect::<Vec<_>>();
    println!("{}", ans.join(" "));
}

#[derive(Clone)]
struct F(Option<Mint>);

#[derive(Clone)]
struct S {
    size: usize,
    value: Mint,
}

impl MapMonoid for F {
    type M = S;

    type F = F;

    fn identity_map() -> Self::F {
        F(None)
    }

    fn mapping(
        f: &Self::F,
        x: &<Self::M as ac_library::Monoid>::S,
    ) -> <Self::M as ac_library::Monoid>::S {
        match f.0 {
            Some(f) => S {
                size: x.size,
                value: f * x.size,
            },
            None => x.clone(),
        }
    }

    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        match f.0 {
            Some(f) => F(Some(f)),
            None => g.clone(),
        }
    }
}

impl Monoid for S {
    type S = S;

    fn identity() -> Self::S {
        S {
            size: 0,
            value: Mint::new(0),
        }
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        S {
            size: a.size + b.size,
            value: a.value + b.value,
        }
    }
}
