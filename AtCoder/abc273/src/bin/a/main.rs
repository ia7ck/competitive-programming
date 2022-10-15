use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut f = 1;
    for k in 1..=n {
        f *= k;
    }
    println!("{}", f);
}
