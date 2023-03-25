use proconio::input;

fn main() {
    input! {
        n: usize,
        words: [String; n],
    };

    let dict = vec!["and", "not", "that", "the", "you"];
    for w in words {
        if dict.contains(&w.as_str()) {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
