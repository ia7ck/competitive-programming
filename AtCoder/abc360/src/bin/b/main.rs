use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };

    for w in 1..s.len() {
        for c in 1..=w {
            let mut v = Vec::new();
            for chunk in s.chunks(w) {
                if chunk.len() >= c {
                    v.push(chunk[c - 1]);
                }
            }
            if v == t {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
