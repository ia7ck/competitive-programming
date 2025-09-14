use proconio::input;

fn main() {
    input! {
        x: u64,
        c: u64,
    };

    let mut ans = 0;
    for k in (1000..=x).step_by(1000) {
        if k + (k / 1000) * c <= x {
            ans = k;
        }
    }
    println!("{}", ans);
}
