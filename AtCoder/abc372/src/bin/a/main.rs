use proconio::input;

fn main() {
    input! {
        s: String,
    };

    let ans = s.replace(".", "");
    println!("{}", ans);
}
