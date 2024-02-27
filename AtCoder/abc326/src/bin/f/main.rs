use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        tx: i64,
        ty: i64,
        a: [i64; n],
    };

    let mut dy = Vec::new();
    let mut dx = Vec::new();
    for i in 0..n {
        if i % 2 == 0 {
            dy.push(a[i]);
        } else {
            dx.push(a[i]);
        }
    }

    let (Some(y_sign), Some(x_sign)) = (solve(&dy, ty), solve(&dx, tx)) else {
        println!("No");
        return;
    };

    let mut ans = String::new();
    let mut dir = 0; // 0: +x, 1: +y, 2: -x, 3: -y
    for i in 0..n {
        let c = if i % 2 == 0 {
            match (y_sign[i / 2], dir) {
                (1, 0) | (-1, 2) => 'L',
                (1, 2) | (-1, 0) => 'R',
                _ => unreachable!(),
            }
        } else {
            match (x_sign[(i - 1) / 2], dir) {
                (1, 1) | (-1, 3) => 'R',
                (1, 3) | (-1, 1) => 'L',
                _ => unreachable!(),
            }
        };

        match c {
            'L' => dir = (dir + 1) % 4,
            'R' => dir = (4 + dir - 1) % 4,
            _ => unreachable!(),
        }

        ans.push(c);
    }

    println!("Yes");
    println!("{}", ans);
}

fn solve(a: &[i64], s: i64) -> Option<Vec<i64>> {
    let n = a.len();
    assert!(n <= 40);

    let mut memo = HashMap::new();
    for bits in 0..(1 << (n / 2)) {
        // 0 -> -1
        // 1 -> +1
        let mut sum = 0;
        for i in 0..(n / 2) {
            if bits >> i & 1 == 0 {
                sum -= a[i];
            } else {
                sum += a[i];
            }
        }
        memo.insert(sum, bits);
    }

    for bits in 0..(1 << (n - n / 2)) {
        let mut sum = 0;
        for i in 0..(n - n / 2) {
            if bits >> i & 1 == 0 {
                sum -= a[i + n / 2];
            } else {
                sum += a[i + n / 2];
            }
        }
        if let Some(&bits0) = memo.get(&(s - sum)) {
            let mut sign = Vec::new();
            for i in 0..(n / 2) {
                if bits0 >> i & 1 == 0 {
                    sign.push(-1);
                } else {
                    sign.push(1);
                }
            }
            for i in 0..(n - n / 2) {
                if bits >> i & 1 == 0 {
                    sign.push(-1);
                } else {
                    sign.push(1);
                }
            }
            return Some(sign);
        }
    }
    None
}
