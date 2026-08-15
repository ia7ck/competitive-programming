use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };

    let mut set = BTreeSet::new();
    for a in a {
        set.insert(a);
    }
    let mut cur = 0;
    let mut ans = 0;
    for _ in 0..n {
        let left = set.range(..cur).last().copied();
        let right = set.range(cur..).next().copied();
        match (left, right) {
            (None, None) => unreachable!(),
            (None, Some(r)) => {
                ans += r - cur;
                cur = r;
                set.remove(&r);
            }
            (Some(l), None) => {
                ans += cur - l;
                cur = l;
                set.remove(&l);
            }
            (Some(l), Some(r)) => {
                let dl = cur - l;
                let dr = r - cur;
                if dl <= dr {
                    ans += dl;
                    cur = l;
                    set.remove(&l);
                } else {
                    ans += dr;
                    cur = r;
                    set.remove(&r);
                }
            }
        }
    }

    println!("{}", ans);
}
