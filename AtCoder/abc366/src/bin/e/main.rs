use proconio::input;

fn main() {
    input! {
        n: usize,
        d: i64,
        points: [(i64, i64); n],
    };

    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for (x, y) in points {
        xs.push(x);
        ys.push(y);
    }
    xs.sort();
    ys.sort();

    let med_y = ys[n / 2];
    let min_cost_y = ys.iter().map(|&y| (y - med_y).abs()).sum::<i64>();

    let mut sum_x = vec![0; n + 1];
    let mut sum_y = vec![0; n + 1];
    for i in 0..n {
        sum_x[i + 1] = sum_x[i] + xs[i];
        sum_y[i + 1] = sum_y[i] + ys[i];
    }

    let mut ans = 0;
    const L: i64 = 2_000_000 + 1;
    for x in -L..=L {
        let p = xs.partition_point(|&xx| xx < x);
        let cost_x = (x * p as i64 - sum_x[p]) + ((sum_x[n] - sum_x[p]) - x * (n - p) as i64);
        if cost_x + min_cost_y > d {
            continue;
        }
        let low = {
            let mut ok = med_y;
            let mut ng = -L * 2;
            while ok - ng > 1 {
                let mid = (ok + ng) / 2;
                let p = ys.partition_point(|&yy| yy < mid);
                let cost_y =
                    (mid * p as i64 - sum_y[p]) + ((sum_y[n] - sum_y[p]) - mid * (n - p) as i64);
                if cost_x + cost_y <= d {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            ok
        };
        let high = {
            let mut ok = med_y;
            let mut ng = L * 2;
            while ng - ok > 1 {
                let mid = (ok + ng) / 2;
                let p = ys.partition_point(|&yy| yy < mid);
                let cost_y =
                    (mid * p as i64 - sum_y[p]) + ((sum_y[n] - sum_y[p]) - mid * (n - p) as i64);
                if cost_x + cost_y <= d {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            ok
        };
        ans += high - low + 1;
    }
    println!("{}", ans);
}
