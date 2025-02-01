use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        w: usize,
        xy: [(Usize1, u64); n],
        q: usize,
        ta: [(u64, Usize1); q],
    };

    let (level, block_by_level) = {
        let mut y_by_x = vec![vec![]; w];
        for (i, &(x, y)) in xy.iter().enumerate() {
            y_by_x[x].push((y, i));
        }
        for x in 0..w {
            y_by_x[x].sort_unstable();
        }

        let mut level = vec![0; n];
        let mut block_by_level = vec![vec![]; n];
        for x in 0..w {
            for (l, &(_, i)) in y_by_x[x].iter().enumerate() {
                level[i] = l;
                block_by_level[l].push(i);
            }
        }
        (level, block_by_level)
    };

    const INF: u64 = u64::MAX / 2;
    let mut tl = vec![INF; n];
    for l in 0..n {
        if block_by_level[l].len() < w {
            break;
        }
        tl[l] = block_by_level[l].iter().map(|&i| xy[i].1).max().unwrap() - 1;
        if l >= 1 {
            tl[l] = tl[l].max(tl[l - 1] + 1);
        }
    }

    for (t, a) in ta {
        let tt = tl[level[a]];
        if tt < t {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}
