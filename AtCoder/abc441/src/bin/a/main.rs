use proconio::input;

fn main() {
    input! {
        p: u32,
        q: u32,
        x: u32,
        y: u32,
    };

    if p <= x && x <= p + 100 - 1 && q <= y && y <= q + 100 - 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
