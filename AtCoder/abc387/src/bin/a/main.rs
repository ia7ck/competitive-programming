use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    };

    let ans = (a + b).pow(2);
    println!("{}", ans);
}
