use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    };

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let ng = match a[i][j] {
                'W' => a[j][i] != 'L',
                'L' => a[j][i] != 'W',
                'D' => a[j][i] != 'D',
                _ => unreachable!(),
            };
            if ng {
                println!("incorrect");
                return;
            }
        }
    }
    println!("correct");
}
