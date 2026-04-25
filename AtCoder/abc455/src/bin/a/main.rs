use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
    };

    let ans = if a != b && b == c { "Yes" } else { "No" };
    println!("{}", ans);
}
