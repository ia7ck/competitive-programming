use proconio::input;
use sliding_window::{sliding_window_maximum, sliding_window_minimum};

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [u64; n],
    };

    a.sort();

    let win_max = sliding_window_maximum(&a, n - k);
    let win_min = sliding_window_minimum(&a, n - k);

    let ans = win_max
        .iter()
        .zip(&win_min)
        .map(|(max, min)| max - min)
        .min()
        .unwrap();
    println!("{}", ans);
}
