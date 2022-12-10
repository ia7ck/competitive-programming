use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    for x in (0..=n).rev() {
        println!("{}", x);
    }
}
