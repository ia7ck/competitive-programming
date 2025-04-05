use proconio::input;

fn main() {
    input! {
        a: u32,
    };

    if 400 % a == 0 {
        println!("{}", 400 / a);
    } else {
        println!("-1");
    }
}
