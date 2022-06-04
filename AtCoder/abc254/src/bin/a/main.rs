use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    println!("{}{}", n / 10 % 10, n % 10);
}
