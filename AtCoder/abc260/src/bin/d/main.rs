use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n],
    };

    let mut map = BTreeMap::<usize, usize>::new();
    let mut next = vec![0; n + 1];
    let mut ans = vec![-1; n + 1];
    for i in 0..n {
        if let Some((&top, &count)) = map.range(p[i]..).next() {
            map.remove(&top);
            let new_count = count + 1;
            if new_count == k {
                ans[p[i]] = (i + 1) as i32;
                let mut q = top;
                while q > 0 {
                    ans[q] = (i + 1) as i32;
                    q = next[q];
                }
            } else {
                next[p[i]] = top;
                map.insert(p[i], new_count);
            }
        } else {
            if k == 1 {
                ans[p[i]] = (i + 1) as i32;
            } else {
                map.insert(p[i], 1);
            }
        }
    }
    for i in 1..=n {
        println!("{}", ans[i]);
    }
}
