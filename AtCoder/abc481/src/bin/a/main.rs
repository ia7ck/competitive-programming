use proconio::input;

fn main() {
    input! {
        a: u8,
        b: u8,
    };

    let c = a + b;

    println!("{}", c.to_string().len());
}
