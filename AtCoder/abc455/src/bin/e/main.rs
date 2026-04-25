use std::{collections::HashMap, hash::Hash};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    let total = n * (n + 1) / 2;

    let mut ab = vec![0; n + 1];
    let mut bc = vec![0; n + 1];
    let mut ca = vec![0; n + 1];
    for i in 0..n {
        ab[i + 1] = ab[i];
        bc[i + 1] = bc[i];
        ca[i + 1] = ca[i];
        match s[i] {
            'A' => {
                ab[i + 1] += 1;
                ca[i + 1] -= 1;
            }
            'B' => {
                ab[i + 1] -= 1;
                bc[i + 1] += 1;
            }
            'C' => {
                bc[i + 1] -= 1;
                ca[i + 1] += 1;
            }
            _ => unreachable!(),
        }
    }
    let ab_bc = ab
        .iter()
        .zip(&bc)
        .map(|(&ab, &bc)| (ab, bc))
        .collect::<Vec<_>>();

    let abc = f(&ab_bc);
    let ans = total as isize - f(&ab) - f(&bc) - f(&ca) + abc * 3 - abc;

    println!("{}", ans);
}

fn f<K>(ab: &Vec<K>) -> isize
where
    K: Eq + Hash,
{
    let mut res = 0;
    let mut counter = HashMap::new();
    for p in ab {
        counter
            .entry(p)
            .and_modify(|e| {
                res += *e;
                *e += 1;
            })
            .or_insert(1);
    }
    res
}
