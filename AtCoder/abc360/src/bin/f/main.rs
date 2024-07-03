use ac_library::{LazySegtree, MapMonoid, Monoid};
use proconio::input;
use zarts::SortedSeq;

fn main() {
    input! {
        n: usize,
        lr: [(u32, u32); n],
    };

    let mut rects = Vec::new();
    for (l, r) in lr {
        if l >= 1 && l + 1 <= r - 1 {
            rects.push(((0, l + 1), (l - 1, r - 1)));
        }
        if l + 1 <= r - 1 && r + 1 <= 1_000_000_000 {
            rects.push(((l + 1, r + 1), (r - 1, 1_000_000_000)));
        }
    }

    if rects.is_empty() {
        println!("0 1");
        return;
    }

    let mut values = Vec::new();
    for &((l1, r1), (l2, r2)) in &rects {
        values.push(l1);
        values.push(r1);
        values.push(l2);
        values.push(r2);
    }
    let values = SortedSeq::from_iter(values);

    let mut start = vec![vec![]; values.size()];
    let mut end = vec![vec![]; values.size()];
    for &((l1, r1), (l2, r2)) in &rects {
        start[values.ord(&l1)].push((values.ord(&r1), values.ord(&r2)));
        end[values.ord(&l2)].push((values.ord(&r1), values.ord(&r2)));
    }

    let mut ans = 0;
    let mut ans_l = 0;
    let mut ans_r = 0;
    let mut seg = LazySegtree::<F>::from(vec![0; values.size()]);
    for l in 0..values.size() {
        for &(r1, r2) in &start[l] {
            seg.apply_range(r1..=r2, 1);
        }

        let max = seg.all_prod();
        if ans < max {
            ans = max;
            ans_l = l;
            ans_r = seg.max_right(0, |x| x < max);
        }

        for &(r1, r2) in &end[l] {
            seg.apply_range(r1..=r2, -1);
        }
    }

    assert_ne!(ans, 0);
    println!("{} {}", values.at(ans_l), values.at(ans_r));
}

// 区間add, 区間max

struct M;
impl Monoid for M {
    type S = i64;

    fn identity() -> Self::S {
        i64::MIN / 2
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a.max(b)
    }
}

struct F;
impl MapMonoid for F {
    type M = M;
    type F = i64;

    fn identity_map() -> Self::F {
        0
    }

    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        f + x
    }

    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        f + g
    }
}
