use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        a: [u8; n],
    };

    let mut a: VecDeque<u8> = a.into_iter().collect();
    let mut xor = 0;
    while let Some(last) = a.pop_back() {
        match last ^ xor {
            0 => {
                // B
            }
            1 => {
                if let Some(first) = a.pop_front() {
                    // A
                    if (first ^ xor) != 0 {
                        println!("No");
                        return;
                    }
                    a.push_back(last);
                    xor ^= 1;
                } else {
                    println!("No");
                    return;
                }
            }
            _ => unreachable!(),
        }
    }
    println!("Yes");
}
