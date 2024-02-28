use proconio::input;

fn main() {
    input! {
        x_a: i64,
        y_a: i64,
        x_b: i64,
        y_b: i64,
        x_c: i64,
        y_c: i64,
    };

    let d_xy = solve((x_a, y_a), (x_b, y_b), (x_c, y_c));
    let d_yx = solve((y_a, x_a), (y_b, x_b), (y_c, x_c));

    println!("{}", d_xy.min(d_yx));
}

fn solve((x_a, y_a): (i64, i64), (x_b, y_b): (i64, i64), (x_c, y_c): (i64, i64)) -> i64 {
    // 平行移動
    let (x_b, y_b) = (x_b - x_a, y_b - y_a);
    let (x_c, y_c) = (x_c - x_a, y_c - y_a);

    // 反転
    let (x_b, x_c) = if x_b >= 0 { (x_b, x_c) } else { (-x_b, -x_c) };
    let (y_b, y_c) = if y_b >= 0 { (y_b, y_c) } else { (-y_b, -y_c) };

    // (x_a, y_a) = (0, 0)
    assert!(x_b >= 0 && y_b >= 0);
    assert_ne!((x_b, y_b), (0, 0));
    assert_ne!((x_b, y_b), (x_c, y_c));

    // xをそろえる
    let ((x_a, y_a), (x_b, y_b), d) = if x_b < x_c {
        // +--->*
        // |
        // |
        // |
        (
            (x_c - 1, y_b),
            (x_c, y_b),
            if x_b == 0 {
                // 荷物の左から回りこむ
                1 + y_b + x_c
            } else {
                y_b + (x_c - 1)
            },
        )
    } else if x_c < x_b {
        //    *<--+
        //        |
        //        |
        // -------+
        (
            (x_c + 1, y_b),
            (x_c, y_b),
            if y_b == 0 {
                // 荷物の下から回りこむ
                1 + (x_b + 1) + 1 + (x_b - x_c)
            } else {
                (x_b + 1) + y_b + (x_b - x_c)
            },
        )
    } else {
        ((0, 0), (x_b, y_b), 0)
    };

    assert_eq!(x_b, x_c);

    if y_b == y_c {
        d
    } else {
        // yをそろえる
        d + solve((y_a, x_a), (y_b, x_b), (y_c, x_c))
    }
}
