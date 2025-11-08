use proconio::input;

fn main() {
    input! {
        h: u32,
        b: u32,
    };

    let ans = h.saturating_sub(b);
    println!("{}", ans);
}
