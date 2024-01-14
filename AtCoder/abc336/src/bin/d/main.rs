use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut left = vec![0; n];
    left[0] = 1;
    for i in 1..n {
        left[i] = (left[i - 1] + 1).min(a[i]);
    }
    let mut right = vec![0; n];
    right[n - 1] = 1;
    for i in (0..n - 1).rev() {
        right[i] = (right[i + 1] + 1).min(a[i]);
    }
    let mut ans = 1;
    for i in 0..n {
        ans = ans.max(left[i].min(right[i]));
    }
    println!("{}", ans);
}
