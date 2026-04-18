use proconio::input;

fn main() {
    input! {
        l: u32,
        r: u32,
    };

    let ans = r - l + 1;
    println!("{}", ans);
}
