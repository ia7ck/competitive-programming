use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };

    let mut ans = Vec::new();
    let mut v = s.clone();
    for i in 0..s.len() {
        if s[i] > t[i] {
            v[i] = t[i];
            ans.push(v.clone());
        }
    }
    for i in (0..s.len()).rev() {
        if s[i] < t[i] {
            v[i] = t[i];
            ans.push(v.clone());
        }
    }

    println!("{}", ans.len());
    for ans in ans {
        println!("{}", ans.iter().collect::<String>());
    }
}
