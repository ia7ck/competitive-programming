use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
    };

    let ans = a.pow(b as u32) + b.pow(a as u32);
    println!("{}", ans);
}
