use proconio::input;

fn main() {
    input! {
        n: usize,
        mut x: [u32; n * 5],
    };

    x.sort();
    let mut ans = 0.0;
    for i in n..(n * 4) {
        ans += x[i] as f64;
    }

    println!("{}", ans / (n * 3) as f64);
}
