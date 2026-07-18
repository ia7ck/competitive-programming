use proconio::input;

fn main() {
    input! {
        h: u64,
        w: u64,
    };

    // w / (h/100) / (h/100) >= 25
    if w * 100 * 100 >= 25 * h * h {
        println!("Yes");
    } else {
        println!("No");
    }
}
