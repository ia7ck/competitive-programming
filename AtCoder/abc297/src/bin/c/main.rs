use proconio::{input, marker::Bytes};

fn main() {
    input! {
        h: usize,
        w: usize,
        mut s: [Bytes; h],
    };

    for i in 0..h {
        let mut j = 0;
        while j + 1 < w {
            if s[i][j] == b'T' && s[i][j + 1] == b'T' {
                s[i][j] = b'P';
                s[i][j + 1] = b'C';
                j += 2;
            } else {
                j += 1;
            }
        }
    }

    for row in s {
        for b in row {
            print!("{}", b as char);
        }
        print!("\n");
    }
}
