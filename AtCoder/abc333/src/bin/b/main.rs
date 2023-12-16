use proconio::input;

fn edge(s: &str) -> bool {
    (s == "AB" || s == "BC" || s == "CD" || s == "DE" || s == "EA")
    || (s == "BA" || s == "CB" || s == "DC" || s == "ED" || s == "AE")
}

fn main() {
    input! {
        s: String,
        t: String,
    };

    if edge(&s) ^ edge(&t) {
        println!("No");
    } else {
        println!("Yes");
    }
}
