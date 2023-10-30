use proconio::{input, marker::Bytes};

fn g(digits: &Vec<u8>) -> u64 {
    let mut x = 0;
    for &d in digits {
        x = x * 10 + u64::from(d);
    }
    x
}

fn main() {
    input! {
        n: Bytes,
    };

    let mut ans = 0;
    for bits in 0..(1 << n.len()) {
        let mut left = Vec::new();
        let mut right = Vec::new();
        for i in 0..n.len() {
            if bits >> i & 1 == 1 {
                left.push(n[i] - b'0');
            } else {
                right.push(n[i] - b'0');
            }
        }
        left.sort();
        left.reverse();
        right.sort();
        right.reverse();
        let left = g(&left);
        let right = g(&right);
        if left > 0 && right > 0 {
            ans = ans.max(left * right);
        }
    }
    assert_ne!(ans, 0);
    println!("{}", ans);
}
