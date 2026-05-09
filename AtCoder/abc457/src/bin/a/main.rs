use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [u32; n],
        x: Usize1,
    };

    let ans = a[x];
    println!("{}", ans);
}
