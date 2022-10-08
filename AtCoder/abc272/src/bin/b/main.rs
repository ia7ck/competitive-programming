use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
    };

    let mut ok = vec![vec![false; n]; n];

    for _ in 0..m {
        input! {
            k: usize,
            x: [Usize1; k],
        };
        for i in 0..k {
            for j in (i + 1)..k {
                ok[x[i]][x[j]] = true;
                ok[x[j]][x[i]] = true;
            }
        }
    }
    for i in 0..n {
        for j in (i + 1)..n {
            if !ok[i][j] {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
