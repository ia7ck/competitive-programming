use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        ht: [(char, Usize1); q],
    };

    let (mut left, mut right) = (0, 1);
    let mut ans = 0;
    for (h, t) in ht {
        if h == 'L' {
            if left < right {
                if t <= left {
                    ans += left - t;
                } else if t <= right {
                    ans += t - left;
                } else {
                    ans += n - (t - left);
                }
            } else {
                // left > right
                if left <= t {
                    ans += t - left;
                } else if right <= t {
                    ans += left - t;
                } else {
                    ans += n - (left - t);
                }
            }
            left = t;
        } else {
            if left < right {
                if right <= t {
                    ans += t - right;
                } else if left <= t {
                    ans += right - t;
                } else {
                    ans += n - (right - t);
                }
            } else {
                // left > right
                if t <= right {
                    ans += right - t;
                } else if t <= left {
                    ans += t - right;
                } else {
                    ans += n - (t - right);
                }
            }
            right = t;
        }
    }
    println!("{}", ans);
}
