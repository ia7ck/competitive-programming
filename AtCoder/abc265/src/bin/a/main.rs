use proconio::input;

fn main() {
    input! {
        x: u32,
        y: u32,
        n: u32,
    };

    let ans = if x * 3 <= y {
        n * x
    } else {
        (n / 3) * y + (n % 3) * x
    };
    println!("{}", ans);
}
