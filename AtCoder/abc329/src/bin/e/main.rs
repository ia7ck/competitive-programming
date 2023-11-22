use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        t: Chars,
    };

    // u[..=i]==s[..=i] && u[i]==t[j] となる u があるか
    // i より後は s と一致してなくてもよい
    let mut dp = vec![vec![false; m]; n];
    if s[0] == t[0] {
        dp[0][0] = true;
    }
    // O(nm^3)
    for i in 0..(n - 1) {
        for j in 0..m {
            if dp[i][j] {
                for k in 0..m {
                    if j + 1 == k && s[i + 1] == t[k] {
                        dp[i + 1][k] = true;
                    }

                    //          i
                    // u = *****α
                    // t =   ***α     (j = 3)
                    // t =    **α*    (j = 2)
                    // t =     *α**   (j = 1)
                    // t =      α***  (j = 0)
                    //
                    // u = *****αβ
                    // t =    ***β    (k = 3)
                    // t =     **β*   (k = 2)
                    // t =      *β**  (k = 1)
                    // t =       β*** (k = 0)

                    // α → β
                    if i + 1 >= k && s[(i + 1 - k)..=(i + 1)] == t[..=k] {
                        dp[i + 1][k] = true;
                    }

                    // β → α
                    if j + 1 == m && s[i + 1] == t[k] {
                        dp[i + 1][k] = true;
                    }
                }
            }
        }
    }

    if dp[n - 1][m - 1] {
        println!("Yes");
    } else {
        println!("No");
    }
}
