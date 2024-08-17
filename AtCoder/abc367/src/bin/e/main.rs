use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        mut k: usize,
        mut x: [Usize1; n],
        mut a: [u32; n],
    };

    while k > 0 {
        if k % 2 == 1 {
            a = {
                let mut b = vec![0; n];
                for i in 0..n {
                    b[i] = a[x[i]];
                }
                b
            };
        }
        k /= 2;
        x = {
            let mut y = vec![0; n];
            for i in 0..n {
                y[i] = x[x[i]];
            }
            y
        };
    }

    println!(
        "{}",
        a.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

// 1 2 3 5 7 9 11
// 7 2 9 3 1 5  9
// 1 2 5 9 7 3  5
// 7 2 3 5 1 9  3

// 5 2 6 3 1 4 6
// 1 2 4 6 5 3 4
