use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
    };

    if (a == b && b == c) || (a + b == c) || (b + c == a) || (a + c == b) {
        println!("Yes");
    } else {
        println!("No");
    }
}
