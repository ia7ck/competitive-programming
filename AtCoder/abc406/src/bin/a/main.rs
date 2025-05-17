use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
        d: u32,
    };

    if (c, d) < (a, b) {
        println!("Yes");
    } else {
        println!("No");
    }
}
