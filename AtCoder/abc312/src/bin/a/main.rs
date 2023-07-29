use proconio::input;

fn main() {
    input! {
        s: String,
    };

    let words = ["ACE", "BDF", "CEG", "DFA", "EGB", "FAC", "GBD"];
    if words.contains(&s.as_str()) {
        println!("Yes");
    } else {
        println!("No");
    }
}
