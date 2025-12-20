use rustc_hash::FxHashMap;
use std::cmp::Ordering;

use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(usize, u32); n],
    };

    let mut xy = xy;
    xy.insert(0, (0, 0));

    let mut lens = vec![0; n + 1];
    for (i, &(x, _)) in xy.iter().enumerate() {
        if i == 0 {
            continue;
        }
        lens[i] = lens[x] + 1;
    }

    let mut dx = vec![[0; 32]; n + 1];
    for i in 1..=n {
        dx[i][0] = xy[i].0;
    }
    for i in 1..=n {
        for j in 1..32 {
            let x = dx[i][j - 1];
            dx[i][j] = dx[x][j - 1];
        }
    }

    let mut p = (1..=n).collect::<Vec<_>>();
    let mut memo = FxHashMap::default();
    p.sort_by(|&i, &j| cmp(i, j, &xy, &lens, &dx, &mut memo));
    println!(
        "{}",
        p.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

fn cmp(
    i: usize,
    j: usize,
    xy: &Vec<(usize, u32)>,
    lens: &Vec<usize>,
    dx: &Vec<[usize; 32]>,
    memo: &mut FxHashMap<(usize, usize), Ordering>,
) -> Ordering {
    if i == j {
        return Ordering::Equal;
    }
    if i == 0 {
        return Ordering::Less;
    }
    if j == 0 {
        return Ordering::Greater;
    }

    if let Some(o) = memo.get(&(i, j)) {
        return *o;
    }

    let (xi, yi) = xy[i];
    let (xj, yj) = xy[j];
    let o = match lens[i].cmp(&lens[j]) {
        Ordering::Less => {
            let delta = lens[j] - lens[i];
            let mut xj = j;
            for k in 0..32 {
                if delta >> k & 1 == 1 {
                    xj = dx[xj][k];
                }
            }
            assert_eq!(lens[i], lens[xj]);
            cmp(i, xj, xy, lens, dx, memo).then(Ordering::Less)
        }
        Ordering::Greater => {
            let delta = lens[i] - lens[j];
            let mut xi = i;
            for k in 0..32 {
                if delta >> k & 1 == 1 {
                    xi = dx[xi][k];
                }
            }
            assert_eq!(lens[xi], lens[j]);
            cmp(xi, j, xy, lens, dx, memo).then(Ordering::Greater)
        }
        Ordering::Equal => cmp(xi, xj, xy, lens, dx, memo).then_with(|| yi.cmp(&yj)),
    };
    memo.insert((i, j), o);
    o
}
