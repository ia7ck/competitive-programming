use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut pos = Vec::new();
    for i in 0..s.len() {
        if s[i] == '#' {
            pos.push(i + 1);
        }
    }

    for chunk in pos.chunks(2) {
        assert_eq!(chunk.len(), 2);
        let ans = format!("{},{}", chunk[0], chunk[1]);
        println!("{}", ans);
    }
}
