use std::cmp::Ordering;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        xy: [(i64, i64); n],
        ab: [(Usize1, Usize1); q],
    };

    let mut ord = (0..n).collect::<Vec<_>>();
    ord.sort_unstable_by_key(|&i| {
        let (x, y) = xy[i];
        x.pow(2) + y.pow(2)
    });
    // stable
    ord.sort_by(|&i, &j| cmp(xy[i], xy[j]));

    let mut inv_ord = vec![0; n];
    for i in 0..n {
        inv_ord[ord[i]] = i;
    }

    let mut eq_left = vec![0; n];
    for i in 1..n {
        eq_left[i] = match cmp(xy[ord[i - 1]], xy[ord[i]]) {
            Ordering::Equal => eq_left[i - 1],
            Ordering::Less => i,
            _ => unreachable!(),
        };
    }
    let mut eq_right = vec![0; n];
    eq_right[n - 1] = n - 1;
    for i in (0..(n - 1)).rev() {
        eq_right[i] = match cmp(xy[ord[i]], xy[ord[i + 1]]) {
            Ordering::Equal => eq_right[i + 1],
            Ordering::Less => i,
            _ => unreachable!(),
        };
    }

    for (a, b) in ab {
        let ia = inv_ord[a];
        let ib = inv_ord[b];
        if cmp(xy[a], xy[b]).is_eq() {
            let ans = eq_right[ia] - eq_left[ib] + 1;
            println!("{}", ans);
        } else if ia < ib {
            let left = eq_right[ia] + 1;
            let right = n - eq_left[ib];
            let ans = left + right;
            println!("{}", ans);
        } else {
            let ans = eq_right[ia] - eq_left[ib] + 1;
            println!("{}", ans);
        }
    }
}

// https://ngtkana.hatenablog.com/entry/2021/11/13/202103
fn cmp((x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> Ordering {
    ((y1, x1) < (0, 0))
        .cmp(&((y2, x2) < (0, 0)))
        .then((x2 * y1).cmp(&(x1 * y2)))
}
