use proconio::input;

fn main() {
    input! {
        n: u64,
    };
    if n <= 1000 - 1 {
        println!("{}", n);
        return;
    }
    let mut t = 1000;
    while t * 10 <= n {
        t *= 10;
    }
    assert!(t <= n);
    assert!(n <= t * 10 - 1);

    let ans = n / (t / 100) * (t / 100);
    println!("{}", ans);
}
