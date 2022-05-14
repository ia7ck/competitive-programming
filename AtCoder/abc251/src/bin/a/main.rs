use proconio::input;

fn main() {
    input! {
        s: String,
    };

    for _ in 0..(6 / s.len()) {
        print!("{}", s);
    }
    println!();
}
