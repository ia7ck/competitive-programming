use proconio::input;

fn main() {
    input! {
        x: u32,
    };

    for a in 1..=6 {
        for b in 1..=6 {
            for c in 1..=6 {
                if a + b + c == x {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
}
