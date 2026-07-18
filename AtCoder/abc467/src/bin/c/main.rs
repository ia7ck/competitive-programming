use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [u32; n],
        b: [u32; n - 1],
    };

    assert_eq!(m, 2);

    let mut dp0 = 0;
    let mut dp1 = 0;
    if a[0] == 0 {
        dp1 += 1;
    } else {
        dp0 += 1;
    }
    for i in 1..n {
        (dp0, dp1) = match (a[i], b[i - 1]) {
            (0, 0) => (dp0, dp1 + 1),
            (0, 1) => (dp1, dp0 + 1),
            (1, 0) => (dp0 + 1, dp1),
            (1, 1) => (dp1 + 1, dp0),
            _ => unreachable!(),
        };
    }

    println!("{}", dp0.min(dp1));
}
