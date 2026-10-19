use ac_library::{LazySegtree, MapMonoid, ModInt998244353, Monoid};
use proconio::{input, marker::Usize1};

type Mint = ModInt998244353;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u32; n],
    };

    let mut seg = LazySegtree::<F>::new(n);
    for i in 0..n {
        seg.set(i, news(a[i]));
    }

    for _ in 0..q {
        input! {
            op: u8,
        };

        if op == 1 {
            input! {
                l: Usize1,
                r: Usize1,
                x: u32,
            };

            seg.apply_range(l..=r, Mint::new(x));
        } else {
            input! {
                l: Usize1,
                r: Usize1,
                k: usize,
            };

            let ans = seg.prod(l..=r);
            println!("{}", ans[k]);
        }
    }
}

struct F;
impl MapMonoid for F {
    type M = M;

    type F = Mint;

    fn identity_map() -> Self::F {
        Mint::new(0)
    }

    fn mapping(
        f: &Self::F,
        x: &<Self::M as ac_library::Monoid>::S,
    ) -> <Self::M as ac_library::Monoid>::S {
        [
            // Σ1
            x[0],
            // Σ(s+a) = Σs + aΣ1
            x[1] + f * x[0],
            // Σ(s+a)^2 = Σs^2 + 2aΣs + a^2Σ1
            x[2] + f * x[1] * 2 + f.pow(2) * x[0],
            // Σ(s+a)^3 = Σs^3 + 3aΣs^2 + 3a^2Σs + a^3Σ1
            x[3] + f * x[2] * 3 + f.pow(2) * x[1] * 3 + f.pow(3) * x[0],
            // Σ(s+a)^4 = Σs^4 + 4aΣs^3 + 6a^2Σs^2 + 4a^3Σs + a^4Σ1
            x[4] + f * x[3] * 4 + f.pow(2) * x[2] * 6 + f.pow(3) * x[1] * 4 + f.pow(4) * x[0],
        ]
    }

    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        f + g
    }
}

struct M;
impl Monoid for M {
    type S = [Mint; 5];

    fn identity() -> Self::S {
        [Mint::new(0); 5]
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        std::array::from_fn(|i| a[i] + b[i])
    }
}

fn news(x: u32) -> [Mint; 5] {
    let x = Mint::new(x);
    std::array::from_fn(|i| x.pow(i as u64))
}
