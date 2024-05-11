use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [u32; n],
    };

    if let Some(i) = h[1..].iter().position(|&g| h[0] < g) {
        println!("{}", i + 2);
    } else {
        println!("-1");
    }
}
