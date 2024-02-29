use proconio::input;

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: u64,
            x: u64,
            k: u64,
        };
        solve(n, x, k);
    }
}

fn solve(n: u64, x: u64, k: u64) {
    let k = k.min(128);

    let mut ans = 0;
    let mut last_x = Option::<u64>::None;
    for d in 0..=k {
        let x = x / 2_u64.pow(d as u32);
        if x == 0 {
            break;
        }
        assert!(d <= k);
        let l = count_leaf(n, x, k - d);
        let except = match last_x {
            Some(last_x) => {
                if d == k {
                    0
                } else {
                    count_leaf(n, last_x, k - d - 1)
                }
            },
            None => 0,
        };
        ans += l - except;
        last_x = Some(x);
    }
    println!("{}", ans);
}

fn count_leaf(n: u64, x: u64, k: u64) -> u64 {
    assert!(k <= 128);

    let left = (0..k).fold(Some(x), |acc, _| acc.and_then(|acc| acc.checked_mul(2)));
    let right = (0..k).fold(Some(x), |acc, _| {
        acc.and_then(|acc| acc.checked_mul(2))
            .and_then(|acc| acc.checked_add(1))
    });

    match (left, right) {
        (Some(left), Some(right)) if left <= n => right.min(n) - left + 1,
        _ => 0,
    }
}
