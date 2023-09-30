use proconio::input;

fn main() {
    input! {
        _n: usize,
        s: String,
    };

    if let Some(ans) = s.find("ABC") {
        println!("{}", ans + 1);
    } else {
        println!("-1");
    }
}
