use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    };

    let t = s
        .iter()
        .copied()
        .filter(|ch| match ch {
            'a' | 'i' | 'u' | 'e' | 'o' => false,
            _ => true,
        })
        .collect::<Vec<_>>();

    println!(
        "{}",
        t.iter()
            .map(|ch| ch.to_string())
            .collect::<Vec<_>>()
            .join("")
    );
}
