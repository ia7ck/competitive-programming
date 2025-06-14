use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
        k: u32,
    };

    let ans = a.iter().filter(|&&x| k <= x).count();
    println!("{}", ans);
}
