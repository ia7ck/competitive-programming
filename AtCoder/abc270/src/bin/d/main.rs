use proconio::input;

fn solve(n: usize, takahashi: bool, a: &[usize], memo: &mut Vec<Vec<Option<usize>>>) -> usize {
    if n == 0 {
        return 0;
    }
    if let Some(ans) = memo[n][takahashi as usize] {
        return ans;
    }
    let mut result = 0;
    for &x in a {
        if x <= n {
            let s = solve(n - x, !takahashi, a, memo);
            result = result.max(n - s);
        }
    }
    memo[n][takahashi as usize] = Some(result);
    result
}

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; k],
    };

    let mut memo = vec![vec![None; 2]; n + 1];
    let ans = solve(n, true, &a, &mut memo);
    println!("{}", ans);
}
