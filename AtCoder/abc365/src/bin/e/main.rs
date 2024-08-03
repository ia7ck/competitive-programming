use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    };

    let mut ans = 0;
    for i in 0..30 {
        let b = a.iter().copied().map(|x| x >> i & 1).collect::<Vec<_>>();
        ans += f(&b) * (1 << i);
    }
    for x in a {
        assert!(ans >= x);
        ans -= x;
    }
    println!("{}", ans);
}

// 0/1
fn f(a: &[u64]) -> u64 {
    let mut res = 0;
    let mut acc_xor = 0;
    let mut zero = 1;
    let mut one = 0;
    for &x in a {
        acc_xor ^= x;
        if acc_xor == 0 {
            res += one;
            zero += 1;
        } else {
            res += zero;
            one += 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(f(&[0]), 0);
        assert_eq!(f(&[1]), 1);
        assert_eq!(f(&[0, 0]), 0);
        assert_eq!(f(&[0, 1]), 2);
        assert_eq!(f(&[1, 0]), 2);
        assert_eq!(f(&[1, 1]), 2);
    }
}
