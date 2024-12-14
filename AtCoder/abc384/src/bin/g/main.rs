use ac_library::FenwickTree;
use proconio::{input, marker::Usize1};
use zarts::SortedSeq;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
        q: usize,
        queries: [(Usize1, Usize1); q],
    };

    let mut values = Vec::new();
    for &x in &a {
        values.push(x);
    }
    for &x in &b {
        values.push(x);
    }
    let seq = SortedSeq::from_iter(values);

    let q_sqrt = f64::sqrt(q as f64) as usize;
    let mut ord = (0..q).collect::<Vec<_>>();
    ord.sort_by_key(|&i| {
        let (x, y) = queries[i];
        let j = x * q_sqrt / n;
        (j, if j % 2 == 0 { y } else { n - y })
    });

    struct S {
        abs_sum: i64,
        v_a: FenwickTree<i64>,
        c_a: FenwickTree<i64>,
        v_b: FenwickTree<i64>,
        c_b: FenwickTree<i64>,
    }

    let add_a = |state: &mut S, i: usize| {
        let p = seq.ord(&a[i]);
        let small_v = state.v_b.sum(..p);
        let small_c = state.c_b.sum(..p);
        let large_v = state.v_b.sum(p..);
        let large_c = state.c_b.sum(p..);
        state.abs_sum += a[i] * (small_c as i64) - small_v;
        state.abs_sum += large_v - a[i] * (large_c as i64);
        state.v_a.add(p, a[i]);
        state.c_a.add(p, 1);
    };

    let remove_a = |state: &mut S, i: usize| {
        let p = seq.ord(&a[i]);
        let small_v = state.v_b.sum(..p);
        let small_c = state.c_b.sum(..p);
        let large_v = state.v_b.sum(p..);
        let large_c = state.c_b.sum(p..);
        state.abs_sum -= a[i] * (small_c as i64) - small_v;
        state.abs_sum -= large_v - a[i] * (large_c as i64);
        state.v_a.add(p, -a[i]);
        state.c_a.add(p, -1);
    };

    let add_b = |state: &mut S, i: usize| {
        let p = seq.ord(&b[i]);
        let small_v = state.v_a.sum(..p);
        let small_c = state.c_a.sum(..p);
        let large_v = state.v_a.sum(p..);
        let large_c = state.c_a.sum(p..);
        state.abs_sum += b[i] * (small_c as i64) - small_v;
        state.abs_sum += large_v - b[i] * (large_c as i64);
        state.v_b.add(p, b[i]);
        state.c_b.add(p, 1);
    };

    let remove_b = |state: &mut S, i: usize| {
        let p = seq.ord(&b[i]);
        let small_v = state.v_a.sum(..p);
        let small_c = state.c_a.sum(..p);
        let large_v = state.v_a.sum(p..);
        let large_c = state.c_a.sum(p..);
        state.abs_sum -= b[i] * (small_c as i64) - small_v;
        state.abs_sum -= large_v - b[i] * (large_c as i64);
        state.v_b.add(p, -b[i]);
        state.c_b.add(p, -1);
    };

    let mut state = S {
        abs_sum: 0,
        v_a: FenwickTree::new(seq.size(), 0),
        c_a: FenwickTree::new(seq.size(), 0),
        v_b: FenwickTree::new(seq.size(), 0),
        c_b: FenwickTree::new(seq.size(), 0),
    };
    add_a(&mut state, 0);
    add_b(&mut state, 0);

    let mut ans = vec![0; q];
    let (mut xx, mut yy) = (0, 0);
    for i in ord {
        let (x, y) = queries[i];
        while x < xx {
            remove_a(&mut state, xx);
            xx -= 1;
        }
        while y < yy {
            remove_b(&mut state, yy);
            yy -= 1;
        }
        while xx < x {
            xx += 1;
            add_a(&mut state, xx);
        }
        while yy < y {
            yy += 1;
            add_b(&mut state, yy);
        }
        ans[i] = state.abs_sum;
    }

    for ans in ans {
        println!("{}", ans);
    }
}
