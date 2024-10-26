use proconio::input;

fn main() {
    input! {
        s: String,
    };

    if s.contains("A") && s.contains("B") && s.contains("C") {
        println!("Yes");
    } else {
        println!("No");
    }
}
