use grid_search::around;
use std::collections::BinaryHeap;

use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
        n: usize,
        rce: [(Usize1, Usize1, usize); n],
    };

    let ((si, sj), (gi, gj)) = {
        let mut start = (0, 0);
        let mut goal = (0, 0);
        for i in 0..h {
            for j in 0..w {
                if a[i][j] == 'S' {
                    start = (i, j);
                } else if a[i][j] == 'T' {
                    goal = (i, j);
                }
            }
        }
        (start, goal)
    };

    let mut energy = vec![vec![0; w]; h];
    for (r, c, e) in rce {
        energy[r][c] = e;
    }

    let mut max_e = vec![vec![None; w]; h];
    let mut heap = BinaryHeap::new();
    max_e[si][sj] = Some(energy[si][sj]);
    heap.push((energy[si][sj], (si, sj)));
    while let Some((e, (i, j))) = heap.pop() {
        if e == 0 {
            continue;
        }
        for (ni, nj) in
            around(i, j)
                .y_range(0..h)
                .x_range(0..w)
                .directions(&[(0, 1), (1, 0), (0, -1), (-1, 0)])
        {
            if a[ni][nj] == '#' {
                continue;
            }
            let mut ne = e - 1;
            if ne < energy[ni][nj] {
                ne = energy[ni][nj];
            }
            if max_e[ni][nj] < Some(ne) {
                max_e[ni][nj] = Some(ne);
                heap.push((ne, (ni, nj)));
            }
        }
    }

    if max_e[gi][gj].is_some() {
        println!("Yes");
    } else {
        println!("No");
    }
}
