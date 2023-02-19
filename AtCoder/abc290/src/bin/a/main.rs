use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [u32; n],
        b: [usize; m],
    };

    let mut ans = 0;
    for b in b {
        ans += a[b - 1];
    }

    println!("{}", ans);
}
