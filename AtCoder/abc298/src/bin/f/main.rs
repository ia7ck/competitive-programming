use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        points: [(usize, usize, u64); n],
    };

    let mut row = HashMap::new();
    let mut col = HashMap::new();
    for &(r, c, x) in &points {
        *row.entry(r).or_insert(0) += x;
        *col.entry(c).or_insert(0) += x;
    }
    let mut row = row.into_iter().collect::<Vec<_>>();
    row.sort_by_key(|&(_, v)| v);
    row.reverse();
    let mut col = col.into_iter().collect::<Vec<_>>();
    col.sort_by_key(|&(_, v)| v);
    col.reverse();

    let mut pts = HashMap::new();
    for &(r, c, x) in &points {
        pts.insert((r, c), x);
    }
    let mut ans = 0;
    for &(r, rv) in &row[..2.min(row.len())] {
        for &(c, cv) in &col {
            let s = rv + cv - pts.get(&(r, c)).unwrap_or(&0);
            ans = ans.max(s);
        }
    }
    for &(c, cv) in &col[..2.min(col.len())] {
        for &(r, rv) in &row {
            let s = rv + cv - pts.get(&(r, c)).unwrap_or(&0);
            ans = ans.max(s);
        }
    }
    println!("{}", ans);
}
