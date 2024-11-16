use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    };

    let mut ans = vec![0; n + 1];
    for i in 1..=n {
        ans[i] = 1;
    }
    let mut set = BTreeSet::<(usize, usize, usize)>::new(); // (start, end, color)
    for i in 1..=n {
        set.insert((i, i, i));
    }
    for _ in 0..q {
        input! {
            op: u8,
        };
        if op == 1 {
            input! {
                x: usize,
                c: usize,
            };
            let (start, end, old_c) = set
                .range(..(x, usize::MAX, usize::MAX))
                .last()
                .copied()
                .unwrap();
            set.remove(&(start, end, old_c));
            assert!(ans[old_c] >= end - start + 1);
            ans[old_c] -= end - start + 1;
            ans[c] += end - start + 1;

            // 左とマージ
            let mut new_start = start;
            if let Some((l_start, l_end, l_c)) =
                set.range(..(start, usize::MAX, usize::MAX)).last().copied()
            {
                if l_c == c {
                    new_start = l_start;
                    set.remove(&(l_start, l_end, l_c));
                }
            }

            // 右とマージ
            let mut new_end = end;
            if let Some((r_start, r_end, r_c)) = set.range((end, 0, 0)..).next().copied() {
                if r_c == c {
                    new_end = r_end;
                    set.remove(&(r_start, r_end, r_c));
                }
            }

            set.insert((new_start, new_end, c));
        } else {
            input! {
                c: usize,
            };
            println!("{}", ans[c]);
        }
    }
}
