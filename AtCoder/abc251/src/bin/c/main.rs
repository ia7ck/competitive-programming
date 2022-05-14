use proconio::input;
use std::cmp::Reverse;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        st: [(String, u32); n],
    };

    let mut ti = Vec::new(); // original
    let mut seen = HashSet::new();
    for (i, (s, t)) in st.iter().enumerate() {
        if seen.contains(s) {
            continue;
        }
        seen.insert(s);
        ti.push((*t, i));
    }

    ti.sort_by_key(|&(t, i)| (Reverse(t), i));
    assert!(ti.len() >= 1);
    let (t, ans) = ti[0];
    eprintln!("t = {}", t);
    println!("{}", ans + 1);
}
