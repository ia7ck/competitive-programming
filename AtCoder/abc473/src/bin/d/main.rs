use std::collections::{HashSet, VecDeque};

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
    };

    let mut v = vec![0; n];
    v[0] = k;
    let mut ans = Vec::new();
    let mut seen = HashSet::from([v.clone()]);
    let mut que = VecDeque::from([v]);
    while let Some(v) = que.pop_front() {
        let i = v.iter().position(|&v| v > 0).unwrap();
        for j in (i + 1)..n {
            if (j + 1) % (i + 1) == 0 {
                let m = (j + 1) / (i + 1);
                if v[i] >= m {
                    let mut u = v.clone();
                    u[i] -= m;
                    u[j] += 1;
                    if seen.contains(&u) {
                        continue;
                    }
                    seen.insert(u.clone());
                    que.push_back(u);
                }
            }
        }
        ans.push(v);
    }

    ans.sort_unstable();
    for ans in ans {
        println!(
            "{}",
            ans.iter()
                .map(|ans| ans.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}

// 2 9
//
// 1 4
// 3 3
// 5 2
// 7 1
// 9 0
