use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut black: [i64; n],
        mut white: [i64; m],
    };

    black.sort_unstable();
    black.reverse();
    white.sort_unstable();
    white.reverse();

    let mut ans = 0;
    let mut b_sum = 0;
    let mut w_best = 0;
    for i in 0..n {
        b_sum += black[i];
        if i < m && white[i] >= 0 {
            w_best += white[i];
        }
        ans = ans.max(b_sum + w_best);
    }
    println!("{}", ans);
}
