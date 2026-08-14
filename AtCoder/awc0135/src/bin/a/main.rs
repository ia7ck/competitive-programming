use proconio::input;

fn main() {
    input! {
        n: usize,
        r: u64,
        a: [u64; n],
    };

    let x = a.iter().min().unwrap() + r;
    let ans = a.iter().map(|&a| a + r - x).sum::<u64>();

    println!("{}", ans);
}
