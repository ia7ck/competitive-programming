use proconio::input;

fn main() {
    input! {
        x: u8,
    };

    for y in 1..=3 {
        if x != y {
            println!("{}", y);
            return;
        }
    }

    unreachable!();
}
