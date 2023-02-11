use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: [String; n],
    };

    let mut t = s[..k].iter().collect::<Vec<_>>();
    t.sort();
    for t in t {
        println!("{}", t);
    }
}
