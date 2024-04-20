use proconio::input;

fn main() {
    input! {
        s: String,
    };

    for i in 1..=349 {
        if i == 316 {
            continue;
        }
        let t = format!("ABC{:03}", i);
        if s == t {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
