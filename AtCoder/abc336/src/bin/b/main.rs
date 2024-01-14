use proconio::input;

fn main() {
    input! {
        n: u32,
    };

    let ans = n.trailing_zeros();
    println!("{}", ans);
}
