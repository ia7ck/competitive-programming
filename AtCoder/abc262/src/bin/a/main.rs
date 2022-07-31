use proconio::input;

fn main() {
    input! {
        y: usize,
    };

    let ans = if y % 4 <= 2 {
        y + (2 - y % 4)
    } else {
        y + 3
    };

    println!("{}", ans);
}
