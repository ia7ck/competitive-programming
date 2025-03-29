use proconio::{input, marker::Usize1};

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize,
            a: [Usize1; n * 2],
        };
        solve(n, a);
    }
}

fn solve(n: usize, a: Vec<usize>) {
    let mut pos = vec![vec![]; n];
    for i in 0..(n * 2) {
        pos[a[i]].push(i);
    }
    let mut ans = 0_usize;
    for x in 0..n {
        let px1 = pos[x][0];
        let px2 = pos[x][1];
        if px1.abs_diff(px2) == 1 {
            continue;
        }
        let mut ys = Vec::new();
        for py1 in [px1.wrapping_sub(1), px1 + 1] {
            if py1 < n * 2 {
                ys.push(a[py1]);
            }
        }
        ys.sort_unstable();
        ys.dedup();
        for y in ys {
            if x >= y {
                continue;
            }
            let py1 = pos[y][0];
            let py2 = pos[y][1];
            if py1.abs_diff(py2) == 1 {
                continue;
            }

            if (px1.abs_diff(py1) == 1 && px2.abs_diff(py2) == 1)
                || (px1.abs_diff(py2) == 1 && px2.abs_diff(py1) == 1)
            {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
