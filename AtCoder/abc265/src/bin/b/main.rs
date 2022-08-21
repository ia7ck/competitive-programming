use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut t: u64,
        a: [u64; n - 1],
        xy: [(Usize1, u64); m],
    };

    let mut bonus = vec![0; n];
    for (x, y) in xy {
        bonus[x] = y;
    }
    for i in 0..(n - 1) {
        t += bonus[i];
        if t > a[i] {
            t -= a[i];
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
