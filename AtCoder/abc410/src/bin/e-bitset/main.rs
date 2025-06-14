use bitset_fixed::BitSet;
use proconio::input;

fn main() {
    input! {
        n: usize,
        h: usize,
        m: usize,
        monsters: [(usize, usize); n],
    };

    let mut dp = vec![BitSet::new(m + 1); h + 1];
    dp[h].set(m, true);
    let mut ans = 0;
    for (a, b) in monsters {
        let mut new_dp = vec![BitSet::new(m + 1); h + 1];
        for i in a..=h {
            new_dp[i - a] = dp[i].clone();
        }
        for i in 0..=h {
            new_dp[i] |= &(&dp[i] >> b); // シフト方向に注意
        }

        if new_dp.iter().all(|b| b.count_ones() == 0) {
            break;
        }
        dp = new_dp;
        ans += 1;
    }

    println!("{}", ans);
}
