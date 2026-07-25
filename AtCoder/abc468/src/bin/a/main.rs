use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    };

    let mut ans = 0;
    for i in 0..(n - 2) {
        if a[i] < a[i + 1] && a[i + 1] > a[i + 2] {
            ans += 1;
        }
    }

    println!("{}", ans);
}
