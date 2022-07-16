use join::Join;
use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut x = Vec::new();
    if n % 4 > 0 {
        x.push(n % 4);
    }
    x.extend(vec![4; n / 4]);

    let m = x.iter().map(|d| d * 2).sum::<usize>();
    println!("{}", m);
    println!("{}", x.iter().join(""));
}
