use ac_library::{ModInt998244353, Monoid, Segtree};
use proconio::{
    input,
    marker::{Chars, Usize1},
};

type Mint = ModInt998244353;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u32; n],
        s: Chars,
    };

    let mut seg = Segtree::<M>::new(n);
    for i in 0..n {
        seg.set(i, S::new(s[i], a[i]));
    }

    for _ in 0..q {
        input! {
            op: u8,
        };

        if op == 1 {
            input! {
                i: Usize1,
                x: u32,
            };

            seg.set(i, S::new(s[i], x));
        } else {
            input! {
                l: Usize1,
                r: Usize1,
            };

            let ans = seg.prod(l..=r);
            println!("{}", ans.rgb);
        }
    }
}

struct M;
impl Monoid for M {
    type S = S;

    fn identity() -> Self::S {
        S::default()
    }

    fn binary_operation(x: &Self::S, y: &Self::S) -> Self::S {
        S {
            r: x.r + y.r,
            g: x.g + y.g,
            b: x.b + y.b,
            rg: x.rg + y.rg + x.r * y.g,
            gb: x.gb + y.gb + x.g * y.b,
            rgb: x.rgb + y.rgb + x.rg * y.b + x.r * y.gb,
        }
    }
}

#[derive(Clone, Default)]
struct S {
    r: Mint,
    g: Mint,
    b: Mint,
    rg: Mint,
    gb: Mint,
    rgb: Mint,
}

impl S {
    fn new(c: char, x: u32) -> Self {
        let x = Mint::new(x);
        let def = S::default();
        match c {
            'R' => S { r: x, ..def },
            'G' => S { g: x, ..def },
            'B' => S { b: x, ..def },
            _ => unreachable!(),
        }
    }
}
