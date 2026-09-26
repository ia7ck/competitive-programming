use ac_library::{LazySegtree, MapMonoid, Monoid};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        lr: [(Usize1, Usize1); n],
        queries: [(Usize1, Usize1, Usize1, Usize1); q],
    };

    #[derive(Debug, Clone)]
    struct Q {
        i: usize,
        add: bool,
        l: usize,
        r: usize,
    }

    let mut queries_by_x = vec![vec![]; n + 1];
    for i in 0..q {
        let (a, b, c, d) = queries[i];
        queries_by_x[a].push(Q {
            i,
            add: false,
            l: c,
            r: d + 1,
        });
        queries_by_x[b + 1].push(Q {
            i,
            add: true,
            l: c,
            r: d + 1,
        });
    }

    let mut seg = LazySegtree::<AddSumMapMonoid>::new(m);
    for i in 0..m {
        seg.set(i, AddSum { len: 1, sum: 0 });
    }

    let mut ans = vec![0; q];
    for x in 0..=n {
        if x >= 1 {
            let (l, r) = lr[x - 1];
            seg.apply_range(l..(r + 1), 1);
        }

        for q in &queries_by_x[x] {
            let y = seg.prod(q.l..q.r);
            if q.add {
                ans[q.i] += y.sum;
            } else {
                ans[q.i] -= y.sum;
            }
        }
    }

    for ans in ans {
        println!("{}", ans);
    }
}

struct AddSumMapMonoid;

struct AddSumMonoid;

#[derive(Debug, Clone)]
struct AddSum {
    len: i64,
    sum: i64,
}

impl MapMonoid for AddSumMapMonoid {
    type M = AddSumMonoid;

    type F = i64;

    fn identity_map() -> Self::F {
        0
    }

    fn mapping(
        f: &Self::F,
        x: &<Self::M as ac_library::Monoid>::S,
    ) -> <Self::M as ac_library::Monoid>::S {
        AddSum {
            len: x.len,
            sum: x.sum + f * x.len,
        }
    }

    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        f + g
    }
}

impl Monoid for AddSumMonoid {
    type S = AddSum;

    fn identity() -> Self::S {
        AddSum { len: 0, sum: 0 }
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        AddSum {
            len: a.len + b.len,
            sum: a.sum + b.sum,
        }
    }
}
