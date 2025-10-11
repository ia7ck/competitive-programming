use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: u64,
        a: [u64; n],
    };

    if n == 1 {
        let mut ans = 1;
        if a[0] % m == 0 {
            ans += 1;
        }
        println!("{}", ans);
        return;
    }

    let mut left = Vec::new();
    f(0, n / 2, &mut Vec::new(), &mut left);
    let mut right = Vec::new();
    f(n / 2, n, &mut Vec::new(), &mut right);

    let mut left_0 = HashMap::new();
    let mut left_1 = HashMap::new();
    for left in left {
        let sum = left.iter().map(|&i| a[i]).sum::<u64>() % m;
        if left.last() == Some(&(n / 2 - 1)) {
            *left_0.entry(sum).or_insert(0) += 1;
        } else {
            *left_1.entry(sum).or_insert(0) += 1;
        }
    }

    let mut ans = 0_u64;
    for right in right {
        let sum = right.iter().map(|&i| a[i]).sum::<u64>() % m;
        let key = (m - sum) % m;
        if right.first() == Some(&(n / 2)) {
            ans += left_1.get(&key).unwrap_or(&0);
        } else {
            ans += left_0.get(&key).unwrap_or(&0);
            ans += left_1.get(&key).unwrap_or(&0);
        }
    }

    println!("{}", ans);
}

fn f(i: usize, n: usize, buf: &mut Vec<usize>, acc: &mut Vec<Vec<usize>>) {
    if i >= n {
        acc.push(buf.clone());
        return;
    }

    f(i + 1, n, buf, acc);
    if buf.last().is_some_and(|l| l + 1 == i) {
        // adj
    } else {
        buf.push(i);
        f(i + 1, n, buf, acc);
        buf.pop();
    }
}
