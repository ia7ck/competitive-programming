use proconio::{input, marker::{Bytes}};

fn main() {
    input! {
        s: Bytes,
    };

    let mut count = vec![0_usize; 26];
    let mut diff = 0;
    let mut same = false;
    for b in s {
        for i in 0..26 {
            if i == usize::from(b - b'a') {
                if count[i] >= 1 {
                    same = true;
                }
            } else {
                diff += count[i];
            }
        }
        count[usize::from(b - b'a')] += 1;
    }
    println!("{}", diff + usize::from(same));
}
