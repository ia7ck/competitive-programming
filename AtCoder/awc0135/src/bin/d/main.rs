use proconio::input;

fn main() {
    input! {
        n: usize,
        v: i64,
        xd: [(i64, i64); n],
    };

    const INF: i64 = i64::MAX / 3;
    let mut ng = 0;
    let mut ok = INF;
    while ok - ng > 1 {
        let k = (ng + ok) / 2;
        // [x_i - (v - d_i / k), x_i + (v - d_i / k)]
        //
        // max(left) <= min(right)

        let max_left = xd
            .iter()
            .map(|&(x, d)| x - (v - (d + k - 1) / k))
            .max()
            .unwrap();
        let min_right = xd
            .iter()
            .map(|&(x, d)| x + (v - (d + k - 1) / k))
            .min()
            .unwrap();
        if max_left <= min_right {
            ok = k;
        } else {
            ng = k;
        }
    }

    if ok == INF {
        println!("-1");
    } else {
        println!("{}", ok);
    }
}
