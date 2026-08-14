use memoise::memoise_map;
use proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize,
        k: usize,
    };

    let ans = g(r, k) - g(l - 1, k);
    println!("{}", ans);
}

// 1 <= n <= r
fn g(r: usize, k: usize) -> usize {
    if r == 0 {
        return 0;
    }

    if k == 0 {
        return gz(r)[0];
    }

    h(r, k)
}

// [zero, nonzero]
#[memoise_map(r)]
fn gz(r: usize) -> [usize; 2] {
    if r <= 9 {
        return [0, r];
    }

    let d = r % 10;
    // 0, 1, ..., d
    let rest = gz((r - d) / 10);
    // d+1, d+2, ..., 9
    let carry = gz((r - d) / 10 - 1);

    let zero = (rest[0] * (d + 1) + rest[1]) + (carry[0] * (9 - d));
    let nonzero = 9 + (rest[1] * d) + (carry[1] * (9 - d));
    assert_eq!(zero + nonzero, r);
    [zero, nonzero]
}

#[memoise_map(r, k)]
fn h(r: usize, k: usize) -> usize {
    if r <= 9 {
        return usize::from((1..=r).contains(&k));
    }

    let mut res = usize::from(k <= 9);
    for d in 1..=9 {
        if k % d == 0 {
            if d <= r % 10 {
                res += h(r / 10, k / d);
            } else {
                res += h(r / 10 - 1, k / d);
            }
        }
    }

    res
}
