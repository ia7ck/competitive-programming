use proconio::input;

fn main() {
    input! {
        x: u32,
        y: u32,
    };

    let ans = if x + y <= 12 {
        x + y
    } else {
        x + y - 12
    };

    println!("{}", ans);
}
