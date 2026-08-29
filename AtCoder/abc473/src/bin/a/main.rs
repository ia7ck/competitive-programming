use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    };

    let ans = a[(n / 2)..].iter().sum::<u32>();
    println!("{}", ans);
}
