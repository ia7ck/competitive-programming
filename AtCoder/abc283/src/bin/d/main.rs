use proconio::{input, marker::{Bytes}};

fn main() {
    input! {
        s: Bytes,
    };

    let mut stack = Vec::new();
    let mut used = vec![false; 26];
    let mut cum_sum = vec![vec![0_usize; s.len() + 1]; 26];
    for i in 0..s.len() {
        for e in 0..26 {
            cum_sum[e][i + 1] = cum_sum[e][i];
        }
        if s[i] == b'(' {
            stack.push(i);
        } else if s[i] == b')' {
            let j = stack.pop().unwrap();
            for d in 0..26 {
                let c = cum_sum[d][i + 1] - cum_sum[d][j];
                if c >= 1 {
                    used[d] = false;
                }
            }
        } else {
            let d = (s[i] - b'a') as usize;
            if used[d] {
                println!("No");
                return;
            }
            used[d] = true;
            cum_sum[d][i + 1] += 1;
        }
    }

    println!("Yes");
}
