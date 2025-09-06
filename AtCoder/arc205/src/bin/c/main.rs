use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        st: [(u32, u32); n],
    };

    let mut left = BTreeSet::new();
    let mut right = BTreeSet::new();
    for &(s, t) in &st {
        if s > t {
            left.insert(t);
        }
        if s < t {
            right.insert(s);
        }
    }

    for &(s, t) in &st {
        if s > t {
            if right.range(t..s).next().is_some() {
                println!("No");
                return;
            }
        }
        if s < t {
            if left.range(s..t).next().is_some() {
                println!("No");
                return;
            }
        }
    }

    let mut left = Vec::new();
    let mut right = Vec::new();
    for (i, &(s, t)) in st.iter().enumerate() {
        if s > t {
            left.push((i, s, t));
        }
        if s < t {
            right.push((i, s, t));
        }
    }

    let mut ans = Vec::new();

    left.sort_unstable_by_key(|&(_, _, t)| t);
    let mut r = 0;
    for (i, s, _) in left {
        if s < r {
            println!("No");
            return;
        }
        ans.push(i);
        r = s;
    }

    right.sort_unstable_by_key(|&(_, _, t)| t);
    let mut l = u32::MAX;
    for &(i, s, _) in right.iter().rev() {
        if l < s {
            println!("No");
            return;
        }
        ans.push(i);
        l = s;
    }

    println!("Yes");
    println!(
        "{}",
        ans.iter()
            .map(|x| (x + 1).to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
