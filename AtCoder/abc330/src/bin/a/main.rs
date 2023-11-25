use proconio::input;

fn main() {
    input! {
        n: usize,
        l: u32,
        a: [u32; n],
    };

    let ans = a.iter().filter(|&&x| x >= l).count();
    println!("{}", ans);
}
