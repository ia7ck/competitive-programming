use proconio::input;

fn main() {
    input! {
        r: usize,
        c: usize,
        a: [[u8; 2]; 2],
    };

    println!("{}", a[r - 1][c - 1]);
}
