use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    };

    for i in 1..(n - 1) {
        if s[i - 1] == "sweet" && s[i] == "sweet" {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
