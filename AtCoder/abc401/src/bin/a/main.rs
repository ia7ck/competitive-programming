use proconio::input;

fn main() {
    input! {
        s: u32,
    };

    if 200 <= s && s <= 299 {
        println!("Success");
    } else {
        println!("Failure");
    }
}
