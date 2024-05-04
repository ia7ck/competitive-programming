use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };

    let mut j = 0;
    let mut ans = Vec::new();
    for c in s {
        while j < t.len() && c != t[j] {
            j += 1;
        }
        assert!(j < t.len());
        assert_eq!(c, t[j]);
        ans.push(j);
        j += 1;
    }

    println!(
        "{}",
        ans.iter()
            .map(|&x| (x + 1).to_string())
            .collect::<Vec<_>>()
            .join(" ")
    )
}
