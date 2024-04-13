use proconio::{input, marker::Bytes};

fn main() {
    input! {
        s: Bytes,
        t: Bytes,
    };

    // case 1
    let mut left = vec![vec![false; 26]; s.len()];
    left[0][usize::from(s[0] - b'a')] = true;
    for i in 1..s.len() {
        for j in 0..26 {
            left[i][j] = left[i - 1][j];
        }
        left[i][usize::from(s[i] - b'a')] = true;
    }
    let mut right = vec![vec![false; 26]; s.len()];
    right[s.len() - 1][usize::from(s[s.len() - 1] - b'a')] = true;
    for i in (0..s.len() - 1).rev() {
        for j in 0..26 {
            right[i][j] = right[i + 1][j];
        }
        right[i][usize::from(s[i] - b'a')] = true;
    }

    for i in 1..(s.len() - 1) {
        if s[i].to_ascii_uppercase() == t[1] {
            if left[i - 1][usize::from(t[0] - b'A')] && right[i + 1][usize::from(t[2] - b'A')] {
                println!("Yes");
                return;
            }
        }
    }

    // case 2
    if t[2] == b'X' {
        for i in 0..(s.len() - 1) {
            if s[i].to_ascii_uppercase() == t[0] {
                if right[i + 1][usize::from(t[1] - b'A')] {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
}
