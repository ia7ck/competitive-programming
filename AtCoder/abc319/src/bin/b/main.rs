use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    for i in 0..=n {
        if let Some(j) = (1..=9).find(|&j| n % j == 0 && i % (n / j) == 0) {
            print!("{}", j);
        } else {
            print!("-");
        }
    }
    println!();
}
