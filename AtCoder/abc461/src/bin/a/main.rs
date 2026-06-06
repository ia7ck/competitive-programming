use proconio::input;

fn main() {
    input! {
        a: u32,
        d: u32,
    };

    if a <= d {
        println!("Yes");
    } else {
        println!("No");
    }
}
