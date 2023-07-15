use std::collections::{HashMap, HashSet};

use proconio::input;

fn part(a: &Vec<u32>) -> Vec<(Vec<u32>, Vec<u32>)> {
    assert!(a.is_empty() == false);
    let mut p = Vec::new();
    for bits in 0..(1 << a.len()) {
        let mut b = Vec::new();
        let mut c = Vec::new();
        for i in 0..a.len() {
            if bits >> i & 1 == 1 {
                b.push(a[i]);
            } else {
                c.push(a[i]);
            }
        }
        p.push((b, c));
    }
    p
}

fn solve(
    t: usize,
    a: Vec<u32>,
    pairs: &HashSet<(u32, u32)>,
    memo: &mut HashMap<(usize, Vec<u32>), usize>,
) -> usize {
    // eprintln!("t = {}, a = {:?}", t, a);
    if t == 0 {
        return a.is_empty() as usize;
    }
    if a.is_empty() {
        return 0;
    }
    if let Some(&ans) = memo.get(&(t, a.clone())) {
        return ans;
    }
    let mut ans = 0;
    for (b, c) in part(&a) {
        if b.is_empty() {
            continue;
        }
        let mut bad = false;
        for i in 0..b.len() {
            for j in (i + 1)..b.len() {
                if pairs.contains(&(b[i], b[j])) {
                    bad = true;
                }
            }
        }
        if bad {
            continue;
        }
        ans += solve(t - 1, c, pairs, memo);
    }
    memo.insert((t, a), ans);
    ans
}

fn main() {
    input! {
        n: u32,
        t: usize,
        m: usize,
        pairs: [(u32, u32); m],
    };

    let a = (1..=n).collect::<Vec<_>>();
    let pairs = pairs.into_iter().collect::<HashSet<_>>();
    let mut memo = HashMap::new();
    let ans = solve(t, a, &pairs, &mut memo);
    let mut div = 1;
    for i in 1..=t {
        div *= i;
    }
    assert_eq!(ans % div, 0);
    println!("{}", ans / div);
}
