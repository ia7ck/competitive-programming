use proconio::input;

fn main() {
    input! {
        n: usize,
        ss: [String; n],
    };

    for i in (0..n).rev() {
        println!("{}", ss[i]);
    }
}
