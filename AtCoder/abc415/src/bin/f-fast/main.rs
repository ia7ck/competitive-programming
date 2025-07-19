use ac_library::{Monoid, Segtree};
use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
    };

    let mut seg = Segtree::<S>::new(n);
    for i in 0..n {
        seg.set(i, S::new(s[i]));
    }

    for _ in 0..q {
        input! {
            op: u8,
        };

        if op == 1 {
            input! {
                i: Usize1,
                x: char,
            };
            seg.set(i, S::new(x));
        } else {
            input! {
                l: Usize1,
                r: Usize1,
            };
            let prod = seg.prod(l..=r);
            println!("{}", prod.combo);
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct S {
    l: char,
    r: char,
    l_combo: usize,
    r_combo: usize,
    combo: usize,
    single: bool,
}

impl S {
    fn new(c: char) -> Self {
        S {
            l: c,
            r: c,
            l_combo: 1,
            r_combo: 1,
            combo: 1,
            single: true,
        }
    }
}

impl Monoid for S {
    type S = S;

    fn identity() -> Self::S {
        S {
            l: '?',
            r: '?',
            l_combo: 0,
            r_combo: 0,
            combo: 0,
            single: true,
        }
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        if a == &Self::identity() {
            return b.clone();
        }
        if b == &Self::identity() {
            return a.clone();
        }

        let new_l = a.l;
        let new_r = b.r;

        let mut new_l_combo = a.l_combo;
        if a.single && a.r == b.l {
            new_l_combo += b.l_combo;
        }
        let mut new_r_combo = b.r_combo;
        if b.single && a.r == b.l {
            new_r_combo += a.r_combo;
        }

        let mut new_combo = a.combo.max(b.combo);
        if a.r == b.l {
            new_combo = new_combo.max(a.r_combo + b.l_combo);
        }

        let new_single = a.single && a.r == b.l && b.single;

        S {
            l: new_l,
            r: new_r,
            l_combo: new_l_combo,
            r_combo: new_r_combo,
            combo: new_combo,
            single: new_single,
        }
    }
}
