use proconio::input;

fn main() {
    input! {
        s: String,
    };
    let days = vec!["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];
    let j = days.iter().position(|d| s == d.to_string()).unwrap();
    println!("{}", 5 - j);
}
