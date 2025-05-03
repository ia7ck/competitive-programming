use std::iter;

use proconio::input;

fn main() {
    input! {
        n: usize,
        c: [usize; n - 1],
        a: [usize; n - 1],
    };

    let c = iter::once(0).chain(c).collect::<Vec<_>>();
    let a = iter::once(0).chain(a).collect::<Vec<_>>();

    const INF: usize = usize::MAX / 3;
    // dp[i] : 茶碗iに豆があってi+1,i+2,...に豆がない状態に至る最小操作回数
    let mut dp = vec![INF; n + 1];
    let last = (1..n).rfind(|&i| a[i] == 1).unwrap();
    dp[last] = 0;
    for i in (1..=last).rev() {
        assert_ne!(dp[i], INF);
        for j in ((i - c[i])..i).rev() {
            dp[j] = dp[j].min(dp[i] + 1);
            if a[j] == 1 {
                break;
            }
        }
    }

    println!("{}", dp[0]);
}

// 6
// 1 2 1 3 1
// 1 1 0 1 1

//       2 0
// 3 1 0 0 0
// 3 0 0 0 0
// 0 0 0 0 0

// 16
// 1 1 1 2 5 1 1 3 4 1 4 3 1 1 2
// 1 0 0 0 1 0 0 1 1 0 0 0 0 0 1

//                         1 0 0
//                       1 0 0 0
//                 2 0 0 0 0 0 0
//               3 0 0 0 0 0 0 0
//         4 0 0 0 0 0 0 0 0 0 0
// 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0
// 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
