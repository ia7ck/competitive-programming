use ac_library::{segtree, Monoid};
use proconio::{input, marker::Usize1};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

const P: i64 = 998_244_353;

fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(Usize1, Usize1); q],
    };

    let mut rng = SmallRng::seed_from_u64(123);
    let mut segs = Vec::new();
    for _ in 0..5 {
        segs.push(segtree::Segtree::<Sum>::new(n));
    }
    for (a, b) in ab {
        if segs.iter().all(|seg| seg.prod(a..b) == 0) {
            for i in 0..5 {
                let x = rng.gen_range(0..P);
                segs[i].set(a, x);
                segs[i].set(b, -x);
            }
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

struct Sum;
impl Monoid for Sum {
    type S = i64;

    fn identity() -> Self::S {
        0
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        (*a + *b) % P
    }
}
