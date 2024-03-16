use proconio::input;

fn main() {
    input! {
        x: i64,
    };

    let div10 = x / 10;
    let ans = if x > 0 {
        div10 + i64::from(x % 10 != 0)
    } else {
        div10
    };
    println!("{}", ans);
}
