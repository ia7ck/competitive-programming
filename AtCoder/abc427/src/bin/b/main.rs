use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut a = vec![0_u64; n + 1];
    a[0] = 1;
    for i in 1..=n {
        for j in 0..i {
            a[i] += f(a[j]);
        }
    }
    println!("{}", a[n]);
}

fn f(x: u64) -> u64 {
    let mut x = x;
    let mut res = 0;
    while x > 0 {
        res += x % 10;
        x /= 10;
    }
    res
}
