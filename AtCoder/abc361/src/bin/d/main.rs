use std::collections::{HashMap, VecDeque};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        mut s: Chars,
        mut t: Chars,
    };

    s.push('.');
    s.push('.');
    t.push('.');
    t.push('.');

    let mut que = VecDeque::new();
    let mut dist = HashMap::new();
    que.push_back(s.clone());
    dist.insert(s.clone(), 0);
    while let Some(s) = que.pop_front() {
        let p = s.iter().position(|&c| c == '.').unwrap();
        let mut new_s = s.clone();
        for i in 0..(s.len() - 1) {
            // swap s[i] and s[p]
            // swap s[i+1] and s[p+1]
            if i == p || i == p + 1 || i + 1 == p {
                continue;
            }
            new_s.swap(i, p);
            new_s.swap(i + 1, p + 1);

            if !dist.contains_key(&new_s) {
                dist.insert(new_s.clone(), dist[&s] + 1);
                que.push_back(new_s.clone());
            }

            new_s.swap(i, p);
            new_s.swap(i + 1, p + 1);
        }
    }

    if let Some(ans) = dist.get(&t) {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
