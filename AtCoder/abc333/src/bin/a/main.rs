use proconio::input;

fn main() {
    input! {
        n: u64,
    };

    let mut ans = 0;
    for _ in 0..n {
        ans = ans * 10 + n;
    }
    println!("{}", ans);
}
