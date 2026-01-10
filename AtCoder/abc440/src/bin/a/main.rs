use proconio::input;

fn main() {
    input! {
        x: u32,
        y: u32,
    };

    let ans = x * 2_u32.pow(y);

    println!("{}", ans);
}
