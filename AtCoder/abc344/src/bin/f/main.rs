use proconio::input;

macro_rules! chmin {
    ($a: expr, $b: expr) => {
        $a = $a.min($b);
    };
}

fn main() {
    input! {
        n: usize,
        p: [[u64; n]; n],
        right: [[u64; n - 1]; n],
        down: [[u64; n]; n - 1],
    };

    let mut dp = vec![vec![(u64::MAX, 0); n]; n];
    dp[0][0] = (0, 0);
    for si in 0..n {
        for sj in 0..n {
            let mut cost = vec![vec![u64::MAX / 2; n]; n];
            cost[si][sj] = 0;
            for ti in si..n {
                for tj in sj..n {
                    if (si, sj) == (ti, tj) {
                        continue;
                    }
                    if ti > 0 {
                        chmin!(cost[ti][tj], cost[ti - 1][tj] + down[ti - 1][tj]);
                    }
                    if tj > 0 {
                        chmin!(cost[ti][tj], cost[ti][tj - 1] + right[ti][tj - 1]);
                    }
                    let (action, money) = dp[si][sj];
                    // money + k * p[si][sj] >= cost[ti][tj]
                    let k = if cost[ti][tj] <= money {
                        0
                    } else {
                        (cost[ti][tj] - money + (p[si][sj] - 1)) / p[si][sj]
                    };
                    assert!(money + k * p[si][sj] >= cost[ti][tj]);
                    let new_action = action + k + (ti - si) as u64 + (tj - sj) as u64;
                    let new_money = money + k * p[si][sj] - cost[ti][tj];
                    let (dest_action, dest_money) = dp[ti][tj];
                    if new_action < dest_action || new_action == dest_action && new_money > dest_money {
                        dp[ti][tj] = (new_action, new_money);
                    }
                }
            }
        }
    }

    let (ans, _) = dp[n - 1][n - 1];
    println!("{}", ans);
}
