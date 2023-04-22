use std::io::{self, BufReader};
use proconio::{input, source::line::LineSource};

fn main() {
    let stdin = io::stdin();
    let mut stdin = LineSource::new(BufReader::new(stdin));
    
    input! {
        from &mut stdin,
        n: usize,
    };

    let (mut l, mut r) = (0, n);
    for _ in 0..20 {
        let m = (l + r) / 2;

        println!("? {}", m + 1);
        input! {
            from &mut stdin,
            c: u8,
        };

        if c == 0 {
            l = m;
        } else {
            r = m;
        }
        if l + 1 == r {
            // s[l] = 0 != 1 = s[r]
            println!("! {}", l + 1);
            return;
        }
    }

    unreachable!();
}
