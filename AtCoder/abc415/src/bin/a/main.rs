use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
        x: u32,
    };

    if a.contains(&x) {
        println!("Yes");
    } else {
        println!("No");
    }
}
