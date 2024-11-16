use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        s: Chars,
        q: usize,
        queries: [Usize1; q],
    };

    let mut ans = Vec::new();
    for k in queries {
        let i = k / s.len();
        let j = k % s.len();
        let c = s[j];
        let c = if i.count_ones() % 2 == 0 {
            c
        } else {
            if c.is_ascii_lowercase() {
                c.to_ascii_uppercase()
            } else {
                c.to_ascii_lowercase()
            }
        };
        ans.push(c);
    }

    println!(
        "{}",
        ans.iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
