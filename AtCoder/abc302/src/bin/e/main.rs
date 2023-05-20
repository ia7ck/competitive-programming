use std::collections::HashSet;

use proconio::{input, marker::Usize1, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    };

    let mut ans = n;
    let mut edges = vec![HashSet::new(); n];
    for _ in 0..q {
        input! {
            op: u8,
        };
        if op == 1 {
            input! {
                u: Usize1,
                v: Usize1,
            };
            if edges[u].is_empty() {
                ans -= 1;
            }
            if edges[v].is_empty() {
                ans -= 1;
            }
            edges[u].insert(v);
            edges[v].insert(u);
        } else {
            input! {
                v: Usize1,
            };
            if edges[v].is_empty() == false {
                ans += 1;
            }
            let adj = edges[v].drain().collect::<Vec<_>>();
            for w in adj {
                edges[w].remove(&v);
                if edges[w].is_empty() {
                    ans += 1;
                }
            }
        }
        println!("{}", ans);
    }
}
