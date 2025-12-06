use proconio::input;

fn main() {
    input! {
        n: u32,
    };

    let ans = (1..=n).sum::<u32>();
    println!("{}", ans);
}
