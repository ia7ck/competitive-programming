use std::collections::HashMap;

use itertools::Itertools;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::ToPrimitive;
use proconio::input;

fn main() {
    input! {
        a: [u32; 6],
    };

    let ans = solve(3, vec![], &a, &mut HashMap::new());
    println!("{}", ans.to_f64().unwrap());
}

fn solve(
    k: usize,
    mut keep: Vec<usize>,
    a: &Vec<u32>,
    memo: &mut HashMap<(usize, Vec<usize>), BigRational>,
) -> BigRational {
    assert!(k >= 1);
    assert!(keep.len() <= 5);

    if let Some(res) = memo.get(&(k, keep.clone())) {
        return res.clone();
    }

    let rest = 5 - keep.len();
    let mut res = BigRational::from(BigInt::from(0));
    if k == 1 {
        if rest == 0 {
            res = BigRational::from(BigInt::from(score(&keep, &a)));
        } else {
            for p in (0..rest).map(|_| 0..a.len()).multi_cartesian_product() {
                assert_eq!(p.len(), rest);
                for i in p {
                    keep.push(i);
                }
                res += BigRational::new(score(&keep, &a).into(), 6_u32.pow(rest as u32).into());
                for _ in 0..rest {
                    keep.pop();
                }
            }
        }
    } else {
        if rest == 0 {
            res = solve(k - 1, keep.clone(), a, memo);
        } else {
            for p in (0..rest).map(|_| 0..a.len()).multi_cartesian_product() {
                assert_eq!(p.len(), rest);
                let mut best = BigRational::from(BigInt::from(0));
                for bits in 0..(1 << rest) {
                    for i in 0..rest {
                        if bits >> i & 1 == 1 {
                            keep.push(p[i]);
                        }
                    }
                    best = best.max(solve(k - 1, keep.clone(), a, memo));
                    for i in 0..rest {
                        if bits >> i & 1 == 1 {
                            keep.pop();
                        }
                    }
                }
                res += best / BigRational::from(BigInt::from(6_u32.pow(rest as u32)));
            }
        }
    }

    memo.insert((k, keep), res.clone());
    res
}

fn score(keep: &Vec<usize>, a: &Vec<u32>) -> u32 {
    assert_eq!(keep.len(), 5);
    let mut res = 0;
    for &i in keep {
        let x = a[i];
        let n = keep.iter().filter(|&&j| a[i] == a[j]).count() as u32;
        res = res.max(n * x);
    }
    assert_ne!(res, 0);
    res
}
