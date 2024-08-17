use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
        q: usize,
        queries: [(Usize1, Usize1); q],
    };

    let q_sqrt = f64::sqrt(q as f64) as usize;
    let mut ord = (0..q).collect::<Vec<_>>();
    ord.sort_by_key(|&i| {
        let (l, r) = queries[i];
        let j = l * q_sqrt / n;
        (j, if j % 2 == 0 { r } else { n - r })
    });

    struct S {
        counter: Vec<usize>,
        pair: usize,
    }

    let add = |state: &mut S, i: usize| {
        state.counter[a[i]] += 1;
        if state.counter[a[i]] % 2 == 0 {
            state.pair += 1;
        }
    };

    let remove = |state: &mut S, i: usize| {
        state.counter[a[i]] -= 1;
        if state.counter[a[i]] % 2 == 1 {
            state.pair -= 1;
        }
    };

    let mut state = {
        let mut counter = vec![0; n];
        for i in 0..n {
            counter[a[i]] += 1;
        }
        let mut pair = 0;
        for c in &counter {
            pair += c / 2;
        }
        S { counter, pair }
    };

    let mut ans = vec![0; q];
    let (mut left, mut right) = (0, n - 1);
    for i in ord {
        let (l, r) = queries[i];
        while l < left {
            left -= 1;
            add(&mut state, left);
        }
        while right < r {
            right += 1;
            add(&mut state, right);
        }
        while left < l {
            remove(&mut state, left);
            left += 1;
        }
        while r < right {
            remove(&mut state, right);
            right -= 1;
        }
        ans[i] = state.pair;
    }

    for ans in ans {
        println!("{}", ans);
    }
}
