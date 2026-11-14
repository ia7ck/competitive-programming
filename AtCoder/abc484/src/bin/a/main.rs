use proconio::input;

fn main() {
    input! {
        b: u32,
        r: u32,
    };

    let ans = (30 - b) * (30 - r);
    println!("{}", ans);
}
