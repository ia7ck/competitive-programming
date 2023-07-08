use proconio::input;

fn main() {
    input! {
        a: u8,
        b: u8,
    };

    match (a, b) {
        (1, 2) | (2, 3) | (4, 5) | (5, 6) | (7, 8) | (8, 9) => {
            println!("Yes");
        }
        _ => {
            println!("No");
        }
    }
}
