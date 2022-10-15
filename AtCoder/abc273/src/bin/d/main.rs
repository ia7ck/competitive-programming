use proconio::{fastout, input};
use std::collections::{BTreeSet, HashMap};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        r_s: usize,
        c_s: usize,
        n: usize,
        walls: [(usize, usize); n],
        q: usize,
    };

    let mut rows = HashMap::<usize, BTreeSet<usize>>::new();
    let mut cols = HashMap::<usize, BTreeSet<usize>>::new();
    for (r, c) in walls {
        rows.entry(r).or_default().insert(c);
        cols.entry(c).or_default().insert(r);
    }

    let mut ans = Vec::new();
    let (mut r, mut c) = (r_s, c_s);
    for _ in 0..q {
        input! {
            d: char,
            len: usize,
        };

        match d {
            'L' => {
                if let Some(row) = rows.get(&r) {
                    if let Some(left) = row.range(..c).last() {
                        // c - len
                        c = (left + 1).max(c.saturating_sub(len));
                    } else {
                        c = 1.max(c.saturating_sub(len));
                    }
                } else {
                    c = 1.max(c.saturating_sub(len));
                }
            }
            'R' => {
                if let Some(row) = rows.get(&r) {
                    if let Some(right) = row.range((c + 1)..).next() {
                        // c + len
                        c = (right - 1).min(c + len);
                    } else {
                        c = w.min(c + len);
                    }
                } else {
                    c = w.min(c + len);
                }
            }
            'U' => {
                if let Some(col) = cols.get(&c) {
                    if let Some(up) = col.range(..r).last() {
                        // r - len
                        r = (up + 1).max(r.saturating_sub(len));
                    } else {
                        r = 1.max(r.saturating_sub(len));
                    }
                } else {
                    r = 1.max(r.saturating_sub(len));
                }
            }
            'D' => {
                if let Some(col) = cols.get(&c) {
                    if let Some(down) = col.range((r + 1)..).next() {
                        // r + len
                        r = (down - 1).min(r + len);
                    } else {
                        r = h.min(r + len);
                    }
                } else {
                    r = h.min(r + len);
                }
            }
            _ => {
                unreachable!();
            }
        }
        ans.push((r, c));
    }

    for (r, c) in ans {
        println!("{} {}", r, c);
    }
}
