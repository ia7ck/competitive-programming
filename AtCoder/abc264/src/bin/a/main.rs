use proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize,
    };

    let s: Vec<char> = "atcoder".chars().collect();
    for ch in &s[(l - 1)..r] {
        print!("{}", ch);
    }
    println!();
}
