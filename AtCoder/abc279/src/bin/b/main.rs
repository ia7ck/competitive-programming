use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    };

    if s.find(&t).is_some() {
        println!("Yes");
    } else {
        println!("No");
    }
}
