use binary_search_range::BinarySearchRange;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        p: u64,
        a: [u64; n],
        mut b: [u64; m],
    };

    b.sort();
    let mut cum_sum = vec![0; m + 1];
    for i in 0..m {
        cum_sum[i + 1] = cum_sum[i] + b[i];
    }

    let mut ans = 0;
    for x in a {
        let y = p.saturating_sub(x);
        if y > 0 {
            let leq = b.range(1..(y + 1));
            ans += (x * leq.len() as u64) + cum_sum[leq.len()];
        }
        let greater = b.range((y + 1)..1_000_000_000);
        ans += p * greater.len() as u64;
    }

    println!("{}", ans);
}
