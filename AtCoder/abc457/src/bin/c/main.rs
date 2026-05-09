use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: Usize1,
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
        c: [usize; n],
    };

    let mut acc = 0;
    let i = (0..n)
        .find(|&i| {
            let new_acc = acc + a[i].len() * c[i];
            if new_acc > k {
                true
            } else {
                acc = new_acc;
                false
            }
        })
        .unwrap();
    let j = (k - acc) % a[i].len();
    let ans = a[i][j];

    println!("{}", ans);
}
