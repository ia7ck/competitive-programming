use proconio::input;

fn main() {
    input! {
        b: u32,
        g: u32,
    };

    if b > g {
        println!("Bat");
    } else {
        println!("Glove");
    }
}
