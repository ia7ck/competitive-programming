use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u64,
        mut a: [u64; n],
    };

    a.sort_unstable();
    for i in 0..n {
        // a[i] + q * k <= a[n - 1]
        let q = (a[n - 1] - a[i]) / k;
        a[i] += q * k;
        assert!(a[i] <= a[n - 1]);
        assert!(a[i] + k > a[n - 1]);
    }
    a.sort_unstable();

    let mut ans = u64::MAX;
    let mut d = a.into_iter().collect::<VecDeque<_>>();
    for _ in 0..=n {
        ans = ans.min(d.back().unwrap() - d.front().unwrap());

        let x = d.pop_front().unwrap();
        d.push_back(x + k);
    }

    println!("{}", ans);
}
