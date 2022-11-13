use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        p: [usize; n],
    };

    let ans = p.iter().position(|&y| y == x).unwrap();
    println!("{}", ans + 1);
}
