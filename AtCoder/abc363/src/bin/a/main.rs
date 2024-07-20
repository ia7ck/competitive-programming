use proconio::input;

fn main() {
    input! {
        r: u32,
    };

    let t = (r / 100 + 1) * 100;
    let ans = t - r;
    println!("{}", ans);
}
