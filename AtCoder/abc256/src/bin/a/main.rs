use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let ans = 1_usize << n;
    println!("{}", ans);
}
