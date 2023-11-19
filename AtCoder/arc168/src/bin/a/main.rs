use proconio::{input, marker::Chars};

use run_length::RunLength;

fn main() {
    input! {
        _n: usize,
        s: Chars,
    };

    let mut ans = 0;
    for (ch, len) in RunLength::new(s.iter()) {
        if ch == &'>' {
            ans += len * (len + 1) / 2;
        }
    }
    println!("{}", ans);
}

// 30
// <<><>>><><>><><><<>><<<><><<>
//   1 123 1 12 1 1  12   1 1  1
