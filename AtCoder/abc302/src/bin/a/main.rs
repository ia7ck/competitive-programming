use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
    };

    if a % b == 0 {
        println!("{}", a / b);
    } else {
        println!("{}", (a + b - 1) / b);
    }
}
