use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u64; n],
        queries: [(Usize1, Usize1); q],
    };

    let q_sqrt = f64::sqrt(q as f64) as usize;
    let mut ord = (0..q).collect::<Vec<_>>();
    ord.sort_by_key(|&i| {
        let (l, r) = queries[i];
        let j = l * q_sqrt / n;
        (j, if j % 2 == 0 { r } else { n - r })
    });

    #[derive(Debug)]
    struct S {
        // right - left + 1
        len: u64,
        // a[left] + 2 * a[left + 1] + ...
        sx: u64,
        // ... + 2 * a[right - 1] + a[right]
        sy: u64,
        // sum(a[left..=right])
        sum: u64,

        ans: u64,
    }

    // 1
    // 2 2
    // 3 4 3
    // 4 6 6 4
    // 5 8 9 8 5

    let add_left = |state: S, i: usize| -> S {
        let new_len = state.len + 1;
        let new_sx = a[i] + (state.sx + state.sum);
        let new_sy = state.sy + new_len * a[i];
        let new_sum = a[i] + state.sum;
        let new_ans = state.ans + new_sy;
        S {
            len: new_len,
            sx: new_sx,
            sy: new_sy,
            sum: new_sum,
            ans: new_ans,
        }
    };

    let add_right = |state: S, i: usize| -> S {
        let new_len = state.len + 1;
        let new_sx = state.sx + new_len * a[i];
        let new_sy = (state.sy + state.sum) + a[i];
        let new_sum = state.sum + a[i];
        let new_ans = state.ans + new_sx;
        S {
            len: new_len,
            sx: new_sx,
            sy: new_sy,
            sum: new_sum,
            ans: new_ans,
        }
    };

    let remove_left = |state: S, i: usize| -> S {
        let new_len = state.len - 1;
        let new_sx = state.sx - state.sum;
        let new_sy = state.sy - state.len * a[i];
        let new_sum = state.sum - a[i];
        let new_ans = state.ans - state.sy;
        S {
            len: new_len,
            sx: new_sx,
            sy: new_sy,
            sum: new_sum,
            ans: new_ans,
        }
    };

    let remove_right = |state: S, i: usize| -> S {
        let new_len = state.len - 1;
        let new_sx = state.sx - state.len * a[i];
        let new_sy = state.sy - state.sum;
        let new_sum = state.sum - a[i];
        let new_ans = state.ans - state.sx;
        S {
            len: new_len,
            sx: new_sx,
            sy: new_sy,
            sum: new_sum,
            ans: new_ans,
        }
    };

    // [0, 0]
    let mut state = S {
        len: 1,
        sx: a[0],
        sy: a[0],
        sum: a[0],
        ans: a[0],
    };

    let mut ans = vec![0; q];
    // inclusive [left, right]
    let (mut left, mut right) = (0, 0);
    for i in ord {
        let (l, r) = queries[i];
        while l < left {
            left -= 1;
            state = add_left(state, left);
        }
        while right < r {
            right += 1;
            state = add_right(state, right);
        }
        while left < l {
            state = remove_left(state, left);
            left += 1;
        }
        while r < right {
            state = remove_right(state, right);
            right -= 1;
        }
        ans[i] = state.ans;
    }

    for ans in ans {
        println!("{}", ans);
    }
}
