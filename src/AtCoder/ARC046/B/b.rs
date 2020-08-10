extern crate proconio;
use proconio::input;

fn main() {
    input! {
      n: i64,
      a: i64,
      b: i64,
    }
    let tak = "Takahashi";
    let aok = "Aoki";
    if a >= n {
        println!("{}", tak);
        return;
    }
    if a == b {
        println!("{}", if n % (a + 1) != 0 { tak } else { aok });
        return;
    }
    println!("{}", if a > b { tak } else { aok });
}
