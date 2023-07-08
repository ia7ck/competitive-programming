use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [u64; n],
        s: Chars,
    };

    let mut count_m0 = vec![0; n];
    let mut count_m1 = vec![0; n];
    let mut count_m2 = vec![0; n];
    for i in 0..n {
        if i >= 1 {
            count_m0[i] += count_m0[i - 1];
            count_m1[i] += count_m1[i - 1];
            count_m2[i] += count_m2[i - 1];
        }
        if s[i] == 'M' {
            if a[i] == 0 {
                count_m0[i] += 1;
            } else if a[i] == 1 {
                count_m1[i] += 1;
            } else {
                count_m2[i] += 1;
            }
        }
    }
    let mut count_x0 = vec![0; n];
    let mut count_x1 = vec![0; n];
    let mut count_x2 = vec![0; n];
    for i in (0..n).rev() {
        if i + 1 < n {
            count_x0[i] += count_x0[i + 1];
            count_x1[i] += count_x1[i + 1];
            count_x2[i] += count_x2[i + 1];
        }
        if s[i] == 'X' {
            if a[i] == 0 {
                count_x0[i] += 1;
            } else if a[i] == 1 {
                count_x1[i] += 1;
            } else {
                count_x2[i] += 1;
            }
        }
    }

    let mut ans = 0_u64;
    for i in 0..n {
        if s[i] == 'E' {
            if a[i] == 0 {
                // mex(.) = 1
                ans += 1 * (count_m0[i] + count_m2[i]) * (count_x0[i] + count_x2[i]);
                // mex(.) = 2
                ans += 2 * (count_m0[i] * count_x1[i] + count_m1[i] * count_x0[i] + count_m1[i] * count_x1[i]);
                // mex(.) = 3
                ans += 3 * (count_m1[i] * count_x2[i] + count_m2[i] * count_x1[i]);
            } else if a[i] == 1 {
                // mex(.) = 1

                // mex(.) = 2
                ans += 2 * (count_m0[i] * count_x1[i] + count_m1[i] * count_x0[i] + count_m0[i] * count_x0[i]);
                // mex(.) = 3
                ans += 3 * (count_m0[i] * count_x2[i] + count_m2[i] * count_x0[i]);
            } else {
                // mex(.) = 1
                ans += 1 * (count_m0[i] * count_x2[i] + count_m2[i] * count_x0[i] + count_m0[i] * count_x0[i]);
                // mex(.) = 2

                // mex(.) = 3
                ans += 3 * (count_m0[i] * count_x1[i] + count_m1[i] * count_x0[i]);
            }
        }
    }

    println!("{}", ans);
}
