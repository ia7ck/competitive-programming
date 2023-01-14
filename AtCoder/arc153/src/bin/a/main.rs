use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let d1 = (n - 1) / (10 * 10 * 10 * 10 * 10) + 1;
    assert!(1 <= d1 && d1 <= 9);

    let n = (n - 1) % (10 * 10 * 10 * 10 * 10);
    let d3 = n / (10 * 10 * 10 * 10);
    assert!(d3 <= 9);

    let n = n % (10 * 10 * 10 * 10);
    let d4 = n / (10 * 10 * 10);
    assert!(d4 <= 9);

    let n = n % (10 * 10 * 10);
    let d5 = n / (10 * 10);
    assert!(d5 <= 9);

    let n = n % (10 * 10);
    let d7 = n / 10;
    assert!(d7 <= 9);

    let d8 = n % 10;
    assert!(d8 <= 9);

    println!("{}{}{}{}{}{}{}{}{}", d1, d1, d3, d4, d5, d5, d7, d8, d7);
}
