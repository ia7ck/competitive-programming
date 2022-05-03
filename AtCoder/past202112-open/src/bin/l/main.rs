use binary_search_range::BinarySearchRange;
use proconio::input;

fn main() {
    input! {
        n: usize,
        p: i32,
        mut a: [i32; n],
    };

    a.reverse();
    let mut b = Vec::new();
    let mut change = 0;
    for (i, &a) in a.iter().enumerate() {
        let i = i as i32;
        if 0 <= a - i && a - i <= p - (n as i32 - 1) {
            b.push(a - i);
        } else {
            change += 1;
        }
    }

    let mut dp = vec![std::i32::MAX; b.len() + 1];
    dp[0] = 0;
    for &b in &b {
        let j = dp.range(std::i32::MIN..(b + 1)).end;
        assert!(j <= n);
        dp[j] = dp[j].min(b);
    }
    let lis_len = dp.range(std::i32::MIN..std::i32::MAX).end - 1;
    let ans = change + (b.len() - lis_len);
    println!("{}", ans);
}
