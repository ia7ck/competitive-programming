use proconio::input;

fn main() {
    input! {
        s: String,
    };

    if s.chars().filter(|&ch| ch == 'o').count() % 2 == 1 {
        println!("odd");
    } else {
        println!("even");
    }
}
