use proconio::input;

fn main() {
    input! {
        n: usize,
        a: u32,
        b: u32,
        c: [u32; n],
    };

    let ans = c.iter().position(|&c| c == a + b).unwrap();
    println!("{}", ans + 1);
}
