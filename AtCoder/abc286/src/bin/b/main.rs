use proconio::input;

fn main() {
    input! {
        _: usize,
        s: String,
    };

    let t = s.replace("na", "nya");
    println!("{}", t);
}
