use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    };

    let ans = if (a < 0 && b < 0) || (0 < a && 0 < b) {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}
