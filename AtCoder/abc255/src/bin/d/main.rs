use binary_search_range::BinarySearchRange;
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [u64; n],
        xs: [u64; q],
    };

    a.sort();
    let mut cum_sum = vec![0; n + 1];
    for i in 0..n {
        cum_sum[i + 1] = cum_sum[i] + a[i];
    }
    for x in xs {
        let mut ans = 0;
        let less = a.range(0..x);
        ans += less.len() as u64 * x - cum_sum[less.len()];
        let greater = a.range((x + 1)..std::u64::MAX);
        ans += (cum_sum[n] - cum_sum[n - greater.len()]) - greater.len() as u64 * x;
        println!("{}", ans);
    }
}
