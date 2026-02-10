use proconio::input;

use crate::sliding_window::{sliding_window_maximum, sliding_window_minimum};

fn main() {
    input! {
        n: usize,
        k: usize,
        h: [i64; n],
    };

    let mut ans = 0;
    let win_min = sliding_window_minimum(&h, k);
    let win_max = sliding_window_maximum(&h, k);
    for (&&win_min, &&win_max) in win_min.iter().zip(&win_max) {
        ans = ans.max(win_max - win_min);
    }

    println!("{}", ans);
}

// Bundled
#[rustfmt::skip]
#[allow(unused)]
mod sliding_window {
    use std::collections::VecDeque;

    pub fn sliding_window_minimum<T>(a: &[T], window_width: usize) -> Vec<&T>
    where
        T: Ord,
    {
        sliding_window(a, window_width, |last, new| last >= new)
    }

    pub fn sliding_window_maximum<T>(a: &[T], window_width: usize) -> Vec<&T>
    where
        T: Ord,
    {
        sliding_window(a, window_width, |last, new| last <= new)
    }

    fn sliding_window<T, F>(a: &[T], window_width: usize, pop_back_cond: F) -> Vec<&T>
    where
        T: Ord,
        F: Fn(&T, &T) -> bool, // (last, new)
    {
        assert!(0 < window_width && window_width <= a.len());
        let mut result = Vec::new();
        let mut positions: VecDeque<usize> = VecDeque::new();
        for (i, v) in a.iter().enumerate() {
            while let Some(last) = positions.back().copied() {
                if pop_back_cond(&a[last], v) {
                    positions.pop_back();
                } else {
                    break;
                }
            }
            positions.push_back(i);
            if i >= window_width - 1 {
                let p = positions.front().copied().unwrap();
                result.push(&a[p]);
                if p == i - (window_width - 1) {
                    positions.pop_front();
                }
            }
        }
        result
    }
}
