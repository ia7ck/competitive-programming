use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let a = n - n % 5;
    let b = n + (5 - n % 5);

    if n - a < b - n {
        println!("{}", a);
    } else {
        println!("{}", b);
    }
}
