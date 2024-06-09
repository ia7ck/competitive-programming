use proconio::input;

fn main() {
    input! {
        n: usize,
        mut m: usize,
        h: [usize; n],
    };

    for i in 0..n {
        if m < h[i] {
            println!("{}", i);
            return;
        }
        m -= h[i];
    }

    println!("{}", n);
}
