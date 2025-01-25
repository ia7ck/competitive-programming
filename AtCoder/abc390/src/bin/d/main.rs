use std::collections::HashSet;

use proconio::input;
use rand::{rngs::SmallRng, Rng, SeedableRng};

fn main() {
    if cfg!(debug_assertions) {
        let mut rng = SmallRng::seed_from_u64(123);
        for _ in 0..10 {
            let n = rng.gen_range(2..=6);
            let a = (0..n).map(|_| rng.gen_range(1..=10)).collect();
            let ans = solve(n, &a);
            let ans_naive = solve_naive(n, &a);
            assert_eq!(ans, ans_naive);
        }
    }

    input! {
        n: usize,
        a: [u64; n],
    };

    let ans = solve(n, &a);
    println!("{}", ans);
}

fn solve(_n: usize, a: &Vec<u64>) -> usize {
    fn f(a: &[u64], b: &mut Vec<u64>, ans: &mut HashSet<u64>) {
        if a.is_empty() {
            let xor = b.iter().fold(0, |acc, &x| acc ^ x);
            ans.insert(xor);
            return;
        }
        for i in 0..b.len() {
            let old = b[i];
            b[i] += a[0];
            f(&a[1..], b, ans);
            b[i] = old;
        }
        b.push(a[0]);
        f(&a[1..], b, ans);
        b.pop();
    }
    let mut ans = HashSet::new();
    f(a, &mut Vec::new(), &mut ans);
    ans.len()
}

#[allow(unused)]
fn solve_naive(_n: usize, a: &Vec<u64>) -> usize {
    fn f(a: &mut Vec<u64>, ans: &mut HashSet<u64>) {
        let xor = a.iter().fold(0, |acc, &x| acc ^ x);
        ans.insert(xor);
        let n = a.len();
        for i in 0..n {
            if a[i] == 0 {
                continue;
            }
            for j in 0..n {
                if i == j {
                    continue;
                }
                if a[j] == 0 {
                    continue;
                }
                let old_a_i = a[i];
                let old_a_j = a[j];
                a[j] += a[i];
                a[i] = 0;
                f(a, ans);
                a[i] = old_a_i;
                a[j] = old_a_j;
            }
        }
    }

    let mut a = a.clone();
    let mut ans = HashSet::new();
    f(&mut a, &mut ans);
    ans.len()
}
