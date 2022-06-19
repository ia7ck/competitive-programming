use proconio::input;
use std::iter;

fn reverse(n: u64) -> u64 {
    let mut n = n;
    let mut m = 0;
    while n > 0 {
        m = m * 10 + n % 10;
        n /= 10;
    }
    m
}

fn f(x: u64) -> u64 {
    let y = reverse(x);
    x.min(y).min(reverse(y))
}

fn main() {
    assert_eq!(reverse(123), 321);
    assert_eq!(reverse(12300), 321);

    input! {
        n: u64,
        k: u64,
    };

    if k % 10 == 0 {
        println!("0");
    } else {
        let k_rev = reverse(k);
        let mut answer = iter::successors(Some(k), |x| x.checked_mul(10))
            .chain(iter::successors(Some(k_rev), |x| x.checked_mul(10)))
            .filter(|&x| 1 <= x && x <= n && f(x) == k)
            .collect::<Vec<_>>();
        answer.sort();
        answer.dedup();
        println!("{}", answer.len());
    }
}
