use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    };

    if a * 3 > b * 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
