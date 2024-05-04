use std::collections::{HashMap, HashSet};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    };

    let mut a_plus_b = HashSet::new();
    for (a, b) in ab {
        a_plus_b.insert((a + b) % n);
    }
    for i in 0..n {
        if a_plus_b.len() < m {
            if a_plus_b.contains(&i) == false {
                a_plus_b.insert(i);
            }
        }
    }

    let mut ans = Vec::new();
    for i in 0..n {
        for &a_plus_b in &a_plus_b {
            let j = (n + a_plus_b - i) % n;
            assert_eq!((i + j) % n, a_plus_b);
            ans.push((i, j));
        }
    }

    // assert
    {
        let mut row = HashMap::new();
        for &(i, _) in &ans {
            *row.entry(i).or_insert(0) += 1;
        }
        for &count in row.values() {
            assert_eq!(count, m);
        }
        let mut col = HashMap::new();
        for &(_, j) in &ans {
            *col.entry(j).or_insert(0) += 1;
        }
        for &count in col.values() {
            assert_eq!(count, m);
        }
    }

    println!("{}", ans.len());
    for (i, j) in ans {
        println!("{} {}", i + 1, j + 1);
    }
}
