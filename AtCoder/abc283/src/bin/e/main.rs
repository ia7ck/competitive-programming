use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[u8; w]; h],
    };

    const INF: usize = std::usize::MAX;
    let mut dp = [INF; 1 << 2];
    for bits in 0..(1 << 2) {
        let f1 = bits >> 0 & 1 == 1;
        let f2 = bits >> 1 & 1 == 1;
        let mut ok = true;
        for j in 0..w {
            let x = if f1 { 1 - a[0][j] } else { a[0][j] };
            let mut same = false;
            if j >= 1 {
                let l = if f1 { 1 - a[0][j - 1] } else { a[0][j - 1] };
                if l == x {
                    same = true;
                }
            }
            if j + 1 < w {
                let r = if f1 { 1 - a[0][j + 1] } else { a[0][j + 1] };
                if r == x {
                    same = true;
                }
            }
            let d = if f2 { 1 - a[1][j] } else { a[1][j] };
            if d == x {
                same = true;
            }
            ok &= same;
        }
        if ok {
            dp[bits] = (f1 as usize) + (f2 as usize);
        }
    }

    for i in 2..h {
        let mut next = [INF; 1 << 2];
        for bits in 0..(1 << 2) {
            if dp[bits] == INF {
                continue;
            }
            for &e in &[0, 1] {
                let f1 = bits >> 0 & 1 == 1;
                let f2 = bits >> 1 & 1 == 1;
                let mut ok = true;
                for j in 0..w {
                    let x = if f2 { 1 - a[i - 1][j] } else { a[i - 1][j] };
                    let mut same = false;
                    if j >= 1 {
                        let l = if f2 {
                            1 - a[i - 1][j - 1]
                        } else {
                            a[i - 1][j - 1]
                        };
                        if l == x {
                            same = true;
                        }
                    }
                    if j + 1 < w {
                        let r = if f2 {
                            1 - a[i - 1][j + 1]
                        } else {
                            a[i - 1][j + 1]
                        };
                        if r == x {
                            same = true;
                        }
                    }
                    let u = if f1 { 1 - a[i - 2][j] } else { a[i - 2][j] };
                    if u == x {
                        same = true;
                    }
                    let d = if e == 1 { 1 - a[i][j]} else {a[i][j]};
                    if d == x {
                        same = true;
                    }
                    ok &= same;
                }
                let nbits = (e << 1) ^ (f2 as usize);
                if ok {
                    next[nbits] = next[nbits].min(dp[bits] + e);
                }
            }
        }
        dp = next;
    }

    for bits in 0..(1 << 2) {
        if dp[bits] == INF {
            continue;
        }
        let f1 = bits >> 0 & 1 == 1;
        let f2 = bits >> 1 & 1 == 1;
        let mut ok = true;
        for j in 0..w {
            let x = if f2 { 1 - a[h - 1][j] } else { a[h - 1][j] };
            let mut same = false;
            if j >= 1 {
                let l = if f2 {
                    1 - a[h - 1][j - 1]
                } else {
                    a[h - 1][j - 1]
                };
                if l == x {
                    same = true;
                }
            }
            if j + 1 < w {
                let r = if f2 {
                    1 - a[h - 1][j + 1]
                } else {
                    a[h - 1][j + 1]
                };
                if r == x {
                    same = true;
                }
            }
            let u = if f1 { 1 - a[h - 2][j] } else { a[h - 2][j] };
            if u == x {
                same = true;
            }
            ok &= same;
        }
        if ok == false {
            dp[bits] = INF;
        }
    }

    let ans = dp.iter().min().copied().unwrap();
    if ans == INF {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
