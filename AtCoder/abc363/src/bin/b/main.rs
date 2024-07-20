use proconio::input;

fn main() {
    input! {
        n: usize,
        t: u32,
        p: usize,
        mut a: [u32; n],
    };

    let mut ans = 0;
    while a.iter().filter(|&&x| x >= t).count() < p {
        ans += 1;
        for i in 0..n {
            a[i] += 1;
        }
    }
    println!("{}", ans);
}
