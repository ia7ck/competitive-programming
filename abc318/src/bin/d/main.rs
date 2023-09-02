use proconio::input;

const INF: u64 = u64::MAX;

fn solve(n: usize, used: usize, weight: &Vec<Vec<u64>>, memo: &mut Vec<u64>) -> u64 {
    if memo[used] != INF {
        return memo[used];
    }

    let mut ans = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            if used >> i & 1 == 1 || used >> j & 1 == 1 {
                continue;
            }
            ans = ans.max(weight[i][j] + solve(n, used ^ (1 << i) ^ (1 << j), weight, memo));
        }
    }
    memo[used] = ans;
    ans
}

fn main() {
    input! {
        n: usize,
    };
    let mut weight = vec![vec![0; n]; n];
    for i in 0..(n - 1) {
        input! {
            v: [u64; n - i - 1],
        };
        for j in 0..(n - i - 1) {
            weight[i][i + j + 1] = v[j];
            weight[i + j + 1][i] = v[j];
        }
    }

    let ans = solve(n, 0, &weight, &mut vec![INF; 1 << n]);
    println!("{}", ans);
}
