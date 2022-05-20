use proconio::input;

fn main() {
    input! {
        n: usize,
        _a: [u32; n],
    };

    if n % 2 == 0 {
        println!("Second");
    } else {
        println!("First");
    }
}
