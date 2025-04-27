use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    };

    let mut ans = 0;
    for i in (0..n).step_by(2) {
        ans += a[i];
    }
    println!("{}", ans);
}
