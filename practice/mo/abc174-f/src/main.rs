use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        c: [Usize1; n],
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
        uniq: usize,
    }

    let add = |state: &mut S, i: usize| {
        state.counter[c[i]] += 1;
        if state.counter[c[i]] == 1 {
            state.uniq += 1;
        }
    };

    let remove = |state: &mut S, i: usize| {
        state.counter[c[i]] -= 1;
        if state.counter[c[i]] == 0 {
            state.uniq -= 1;
        }
    };

    let mut ans = vec![0; q];
    let mut state = {
        let mut counter = vec![0; n];
        let mut uniq = 0;
        for &c in &c {
            counter[c] += 1;
            if counter[c] == 1 {
                uniq += 1;
            }
        }
        S { counter, uniq }
    };
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
        ans[i] = state.uniq;
    }

    for ans in ans {
        println!("{}", ans);
    }
}
