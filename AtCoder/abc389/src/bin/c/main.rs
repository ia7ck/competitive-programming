use proconio::{input, marker::Usize1};
use fenwick_tree::FenwickTree;

fn main() {
    input! {
        q: usize,
    };

    let mut f = FenwickTree::new(q, 0_u64);
    let (mut next, mut del) = (0, 0);
    for _ in 0..q {
        input! {
            op: u8,
        };
        if op == 1 {
            input! {
                l: u64,
            };
            f.add(next, l);
            next += 1;
        } else if op == 2 {
            del += 1;
        } else {
            input! {
                k: Usize1,
            };
            let ans = f.sum(del..(del + k));
            println!("{}", ans);
        }
    }
}
