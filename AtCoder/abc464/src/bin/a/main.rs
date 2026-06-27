use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let east = s.iter().filter(|&&c| c == 'E').count();
    let west = s.iter().filter(|&&c| c == 'W').count();

    if east > west {
        println!("East");
    } else {
        println!("West");
    }
}
