use proconio::{input, marker::Bytes};

fn main() {
    input! {
        s: [Bytes; 8],
    };

    for i in 0..8 {
        for j in 0..8 {
            if s[i][j] == b'*' {
                let c = (b'a' + j as u8) as char;
                println!("{}{}", c, 8 - i);
                return;
            }
        }
    }

    unreachable!();
}
