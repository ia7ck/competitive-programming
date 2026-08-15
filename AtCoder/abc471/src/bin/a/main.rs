use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    };

    for x in [a + b, a - b, a * b] {
        if x == 9 {
            println!("Nine");
            return;
        }
    }

    // a / b = 9
    if a == b * 9 {
        println!("Nine");
    } else {
        println!("Nein");
    }
}
