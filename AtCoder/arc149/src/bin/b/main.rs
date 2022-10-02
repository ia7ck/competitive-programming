use binary_search_range::BinarySearchRange;
use proconio::input;
use proconio::marker::Usize1;

fn solve(n: usize, a: &[usize], b: &[usize]) -> usize {
    let mut pos_a = vec![0; n];
    for i in 0..n {
        pos_a[a[i]] = i;
    }
    let mut c = Vec::new();
    for x in 0..n {
        c.push(b[pos_a[x]]);
    }
    let inf = std::isize::MAX;
    let mut dp = vec![inf; n + 1];
    dp[0] = -1;
    for x in c {
        let rng = dp.range(-1..(x as isize));
        let j = rng.end;
        dp[j] = dp[j].min(x as isize);
    }
    let mut lis = 1;
    for i in 1..=n {
        if dp[i] < inf {
            lis = i;
        }
    }
    n + lis
}

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
        b: [Usize1; n],
    };

    let ans = solve(n, &a, &b).max(solve(n, &b, &a));
    println!("{}", ans);
}
