use proconio::input;

fn main() {
    input! {
        w: u64,
        b: u64,
    };

    let n = (1..).find(|&n| w * 1000 < n * b).unwrap();
    println!("{}", n);
}
