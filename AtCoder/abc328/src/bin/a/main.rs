use proconio::input;

fn main() {
    input! {
        n: usize,
        x: u32,
        s: [u32; n],
    };

    let ans = s.into_iter().filter(|&y| y <= x).sum::<u32>();
    println!("{}", ans);
}
