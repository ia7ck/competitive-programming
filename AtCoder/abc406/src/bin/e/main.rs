use proconio::input;

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize,
            k: usize,
        };
        solve(n, k);
    }
}

fn solve(n: usize, k: usize) {
    const P: usize = 998244353;

    macro_rules! add {
        ($a:expr, $b:expr) => {
            $a = ($a + $b) % P;
        };
    }

    let n = format!("{n:b}")
        .bytes()
        .map(|b| b - b'0')
        .collect::<Vec<_>>();
    let mut dp = vec![vec![vec![Option::<(usize, usize)>::None; 2]; k + 1]; n.len() + 1];
    dp[0][0][0] = Some((0, 1)); // (sum, count)
    for i in 0..n.len() {
        for j in 0..=k {
            for less in [false, true] {
                let Some((cur_sum, cur_count)) = dp[i][j][usize::from(less)] else {
                    continue;
                };
                let lim = if less { 1 } else { n[i] };
                for d in 0..=lim {
                    let new_j = j + usize::from(d == 1);
                    let new_less = usize::from(less || d < n[i]);
                    if new_j <= k {
                        let (sum, count) = dp[i + 1][new_j][new_less].get_or_insert((0, 0));
                        add!(*sum, cur_sum);
                        if d == 1 {
                            add!(*sum, cur_count * (1 << (n.len() - i - 1)) % P);
                        }
                        add!(*count, cur_count);
                    }
                }
            }
        }
    }

    let mut ans = 0;
    for l in [0, 1] {
        if let Some((sum, _)) = dp[n.len()][k][l] {
            add!(ans, sum);
        }
    }
    println!("{}", ans);
}
