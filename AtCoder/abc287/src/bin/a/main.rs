use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    };

    let f = s.iter().filter(|s| s.as_str() == "For").count();
    if f > n / 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
