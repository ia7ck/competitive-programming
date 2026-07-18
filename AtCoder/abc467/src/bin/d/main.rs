use proconio::input;

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            px: i128,
            py: i128,
            qx: i128,
            qy: i128,
            rx: i128,
            ry: i128,
            sx: i128,
            sy: i128,
        };

        solve(px, py, qx, qy, rx, ry, sx, sy);
    }
}

fn solve(px: i128, py: i128, qx: i128, qy: i128, rx: i128, ry: i128, sx: i128, sy: i128) {
    let rank_a = rank([
        &vec![2 * (qx - px), 2 * (qy - py)],
        &vec![2 * (sx - rx), 2 * (sy - ry)],
    ]);
    let rank_b = rank([
        &vec![
            2 * (qx - px),
            2 * (qy - py),
            (qx.pow(2) + qy.pow(2)) - (px.pow(2) + py.pow(2)),
        ],
        &vec![
            2 * (sx - rx),
            2 * (sy - ry),
            (sx.pow(2) + sy.pow(2)) - (rx.pow(2) + ry.pow(2)),
        ],
    ]);

    if rank_a < rank_b {
        println!("No");
    } else {
        println!("Yes");
    }
}

// 0, 1, 2
fn rank(rows: [&Vec<i128>; 2]) -> usize {
    assert_eq!(rows[0].len(), rows[1].len());

    if rows.iter().all(|row| row.iter().all(|&v| v == 0)) {
        return 0;
    }

    for i in 0..2 {
        let Some(j) = (0..rows[i].len()).find(|&j| rows[i][j] != 0) else {
            continue;
        };

        let u = rows[i][j];
        let v = rows[1 - i][j];

        if (0..rows[i].len()).all(|j| {
            // rows[i][j] * (v / u) = rows[1 - i][j]
            rows[i][j] * v == rows[1 - i][j] * u
        }) {
            return 1;
        }
    }

    2
}
