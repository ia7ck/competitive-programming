use proconio::{input, marker::Chars};
use next_permutation::NextPermutation;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    };

    let mut ord = Vec::new();
    for i in 0..n {
        ord.push(i);
    }
    loop {
        let mut t = Vec::new();
        for &i in &ord {
            t.push(s[i].clone());
        }
        let mut ok = true;
        for i in 0..(n - 1) {
            let mut change = 0;
            for j in 0..m {
                if t[i][j] != t[i + 1][j] {
                    change += 1;
                }
            }
            if change != 1 {
                ok = false;
                break;
            }
        }
        if ok {
            println!("Yes");
            return;
        }
        if ord.next_permutation() == false {
            break;
        }
    }

    println!("No");
}
