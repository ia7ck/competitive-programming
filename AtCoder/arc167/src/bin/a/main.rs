use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [u64; n],
    };

    // sum(a[*]^2) は固定でかかる
    // sum(a[i] * a[j] * 2) を最小化
    // (i, j) を n - m ペア選ばないといけない

    assert!((n - m) * 2 <= n);
    a.sort();
    let b = &a[..(n - m) * 2];
    let mut ans = a.iter().map(|&x| x * x).sum::<u64>();
    for i in 0..(b.len() / 2) {
        ans += b[i] * b[b.len() - 1 - i] * 2;
    }
    println!("{}", ans);
}
