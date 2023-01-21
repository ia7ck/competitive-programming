use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        ab: [(usize, usize); n],
    };

    let mut dp = vec![false; x + 1];
    dp[0] = true;
    for (a, b) in ab {
        for y in (0..x).rev() {
            for c in (a..=(a*b)).step_by(a) {
                if y + c <= x {
                    dp[y + c] |= dp[y];
                }
            }
        }
    }

    if dp[x] {
        println!("Yes");
    } else {
        println!("No");
    }
}
