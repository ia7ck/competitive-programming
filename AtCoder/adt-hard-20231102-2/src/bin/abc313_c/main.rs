use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let total = a.iter().sum::<usize>();
    let mut b = vec![total / n; n];
    for i in 0..n {
        if i < total % n {
            b[i] += 1;
        }
    }
    assert_eq!(total, b.iter().sum());
    let mut a = a;
    a.sort();
    a.reverse();
    let mut ans = 0;
    for i in 0..n {
        ans += a[i].abs_diff(b[i]);
    }
    ans /= 2;
    println!("{}", ans);
}
