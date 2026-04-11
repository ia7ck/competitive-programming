use proconio::input;

fn main() {
    input! {
        _n: usize,
        s: String,
    };

    let ans = s.trim_start_matches('o');
    println!("{}", ans);
}
