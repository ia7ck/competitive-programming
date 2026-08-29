use ac_library::{LazySegtree, MapMonoid, Min};

use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        n: usize,
        mut s: Chars,
        q: usize,
    };

    let mut cumsum = vec![0; n];
    if s[0] == 'A' {
        cumsum[0] = 1;
    } else {
        cumsum[0] = -1
    };
    for i in 1..n {
        cumsum[i] = cumsum[i - 1] + if s[i] == 'A' { 1 } else { -1 };
    }
    let mut seg = LazySegtree::<MinAdd>::from(cumsum);

    for _ in 0..q {
        input! {
            op: u8,
        };

        if op == 1 {
            input! {
                i: Usize1,
                c: char,
            };

            match (s[i], c) {
                ('A', 'B') => {
                    seg.apply_range(i.., -2);
                }
                ('B', 'A') => {
                    seg.apply_range(i.., 2);
                }
                _ => {}
            }
            s[i] = c;
        } else {
            input! {
                l: Usize1,
                r: Usize1,
            };

            let base = if l == 0 { 0 } else { seg.get(l - 1) };
            if base <= seg.prod(l..=r) {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}

struct MinAdd;
impl MapMonoid for MinAdd {
    type M = Min<i32>;
    type F = i32;

    fn identity_map() -> Self::F {
        0
    }

    fn mapping(&f: &i32, &x: &i32) -> i32 {
        f + x
    }

    fn composition(&f: &i32, &g: &i32) -> i32 {
        f + g
    }
}
