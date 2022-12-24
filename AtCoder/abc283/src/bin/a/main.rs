use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u32,
    };

    let ans = a.pow(b);
    println!("{}", ans);
}
