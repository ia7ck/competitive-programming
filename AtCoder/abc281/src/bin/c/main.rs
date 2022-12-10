use proconio::input;

fn main() {
    input! {
        n: usize,
        t: u64,
        a: [u64; n],
    };

    let s = a.iter().sum::<u64>();
    let t = t % s;
    assert!(t >= 1);

    let mut acc = 0;
    for i in 0..n {
        assert!(t > acc);
        assert_ne!(t, acc + a[i]);
        if t < acc + a[i] {
            println!("{} {}", i + 1, t - acc);
            return;
        }
        acc += a[i];
    }

    unreachable!();
}
