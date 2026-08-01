use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };

    let ans = n - k + 1;
    println!("{}", ans);
}
