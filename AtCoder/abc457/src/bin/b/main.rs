use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
    };
    let mut a = Vec::new();
    for _ in 0..n {
        input! {
            l: usize,
            row: [u32; l],
        };
        a.push(row);
    }
    input! {
        x: Usize1,
        y: Usize1,
    };

    let ans = a[x][y];
    println!("{}", ans);
}
