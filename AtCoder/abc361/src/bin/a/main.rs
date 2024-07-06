use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        x: u32,
        a: [u32; n],
    };

    let mut b = a.clone();
    b.insert(k, x);
    for i in 0..n {
        print!("{} ", b[i]);
    }
    println!("{}", b[n]);
}
