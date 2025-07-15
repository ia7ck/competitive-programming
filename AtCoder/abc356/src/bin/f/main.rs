use std::collections::BTreeSet;

use fenwick_tree::FenwickTree;
use proconio::input;
use rand::{rngs::SmallRng, SeedableRng};
use treap::Treap;
use zarts::SortedSeq;

fn main() {
    input! {
        q: usize,
        k: u64,
        queries: [(u8, u64); q],
    };

    let z = queries.iter().map(|&(_, x)| x).collect::<SortedSeq<_>>();
    let mut ft = FenwickTree::new(z.size(), 0);
    let mut pts = BTreeSet::new();
    let mut start = Treap::new(SmallRng::seed_from_u64(123));
    for (op, x) in queries {
        if op == 1 {
            let ix = z.ord(&x);
            if ft.sum(ix..=ix) == 0 {
                // add
                ft.add(ix, 1);
                pts.insert(x);

                let l = start.position(&x).unwrap_err();
                if l == 0 {
                    start.insert(x);
                    if let Some(&rx) = start.nth(1) {
                        if rx - x <= k {
                            start.remove(&rx);
                        }
                    }
                    continue;
                }

                let lx = pts.range(..x).last().copied().unwrap();
                match start.nth(l) {
                    Some(&rx) => {
                        if x - lx <= k && rx - x <= k {
                            start.remove(&rx);
                        } else if x - lx <= k {
                            //
                        } else if rx - x <= k {
                            start.remove(&rx);
                            start.insert(x);
                        } else {
                            start.insert(x);
                        }
                    }
                    None => {
                        if x - lx > k {
                            start.insert(x);
                        }
                    }
                }
            } else {
                // delete
                ft.add(ix, -1);
                pts.remove(&x);

                match start.position(&x) {
                    Ok(_l) => {
                        start.remove(&x);
                        if let Some(&rx) = pts.range(x..).next() {
                            start.insert(rx);
                        }
                    }
                    Err(_l) => match (pts.range(..x).last(), pts.range(x..).next()) {
                        (None, None) => {
                            start.remove(&x);
                        }
                        (None, Some(&rx)) => {
                            start.insert(rx);
                        }
                        (Some(&_lx), None) => {
                            start.remove(&x);
                        }
                        (Some(&lx), Some(&rx)) => {
                            if rx - lx > k {
                                start.insert(rx);
                            }
                        }
                    },
                }
            }
        } else {
            let ans = match start.position(&x) {
                Ok(i) => {
                    let r = start.nth(i + 1).copied();
                    ft.sum(z.ord(&x)..r.map_or(z.size(), |r| z.ord(&r)))
                }
                Err(i) => {
                    assert!(i >= 1);
                    let l = start.nth(i - 1).copied().unwrap();
                    let r = start.nth(i).copied();
                    ft.sum(z.ord(&l)..r.map_or(z.size(), |r| z.ord(&r)))
                }
            };
            println!("{}", ans);
        }
    }
}
