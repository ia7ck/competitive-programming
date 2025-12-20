use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    };

    let ans = a * 12 + b;
    println!("{}", ans);
}
