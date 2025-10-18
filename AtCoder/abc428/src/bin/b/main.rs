use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    };

    let mut f = Vec::new();
    for i in 0..=(n - k) {
        let t = &s[i..(i + k)];
        f.push((t, frequency(&s, t)));
    }

    let x = f.iter().map(|(_, f)| *f).max().unwrap();
    let mut ans = f
        .into_iter()
        .filter_map(|(t, f)| if f == x { Some(t) } else { None })
        .collect::<Vec<_>>();
    ans.sort_unstable();
    ans.dedup();

    println!("{}", x);
    println!(
        "{}",
        ans.iter()
            .map(|t| t.into_iter().collect::<String>())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

fn frequency(s: &[char], t: &[char]) -> usize {
    let mut res = 0;
    for i in 0..=(s.len() - t.len()) {
        if (0..t.len()).all(|j| s[i + j] == t[j]) {
            res += 1;
        }
    }
    res
}
