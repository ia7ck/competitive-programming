use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    };

    let ans = a.iter().sum::<u32>();
    println!("{}", ans);
}
