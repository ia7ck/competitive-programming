use ac_library::{LazySegtree, MapMonoid, Max, Min};
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };

    // TODO: n = 1

    let mut cumsum = vec![0; n + 1];
    for i in 0..n {
        cumsum[i + 1] = cumsum[i] + a[i];
    }
    let mut min_add = LazySegtree::<MinAdd>::new(n + 1);
    let mut max_add = LazySegtree::<MaxAdd>::new(n + 1);
    min_add.set(0, 0);
    max_add.set(0, 0);
    for i in 1..=n {
        min_add.set(i, cumsum[i]);
        max_add.set(i, cumsum[i]);
    }

    let mut ans = max_add.all_prod() - min_add.all_prod() + 1;
    for i in 0..n {
        min_add.apply_range((i + 1).., -a[i] * 2);
        max_add.apply_range((i + 1).., -a[i] * 2);

        let sub = max_add.all_prod() - min_add.all_prod() + 1;
        ans = ans.max(sub);

        min_add.apply_range((i + 1).., a[i] * 2);
        max_add.apply_range((i + 1).., a[i] * 2);
    }
    println!("{}", ans);
}

struct MinAdd;
impl MapMonoid for MinAdd {
    type M = Min<i64>;

    type F = i64;

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

struct MaxAdd;
impl MapMonoid for MaxAdd {
    type M = Max<i64>;

    type F = i64;

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
