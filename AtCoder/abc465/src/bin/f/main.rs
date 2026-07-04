use proconio::{input, marker::Bytes};

fn main() {
    input! {
        n: usize,
        sv: [(Bytes, i64); n],
        q: usize,
        queries: [(Bytes, Bytes); q],
    };

    let mut a = vec![vec![vec![vec![vec![vec![0; 10]; 10]; 10]; 10]; 10]; 10];
    for (s, v) in sv {
        let s = s
            .into_iter()
            .map(|b| usize::from(b - b'0'))
            .collect::<Vec<_>>();
        a[s[0]][s[1]][s[2]][s[3]][s[4]][s[5]] += v;
    }

    for b in 0..=9 {
        for c in 0..=9 {
            for d in 0..=9 {
                for e in 0..=9 {
                    for f in 0..=9 {
                        for g in 0..=9 {
                            cumsum(&mut a, [b, c, d, e, f, g]);
                        }
                    }
                }
            }
        }
    }

    for (x, y) in queries {
        let x = x
            .into_iter()
            .map(|b| usize::from(b - b'0'))
            .collect::<Vec<_>>();
        let y = y
            .into_iter()
            .map(|b| usize::from(b - b'0'))
            .collect::<Vec<_>>();
        if x.iter().zip(&y).any(|(x, y)| x > y) {
            println!("0");
            continue;
        }
        let mut ans = 0;
        let mut z = [None, None, None, None, None, None];
        for bits in 0_usize..(1 << 6) {
            for i in 0..6 {
                if bits >> i & 1 == 0 {
                    z[i] = Some(y[i]);
                } else {
                    z[i] = x[i].checked_sub(1);
                }
            }
            match (z[0], z[1], z[2], z[3], z[4], z[5]) {
                (Some(b), Some(c), Some(d), Some(e), Some(f), Some(g)) => {
                    let sign = if bits.count_ones() % 2 == 0 { 1 } else { -1 };
                    ans += sign * a[b][c][d][e][f][g];
                }
                _ => continue,
            }
        }
        println!("{}", ans);
    }
}

fn cumsum(a: &mut Vec<Vec<Vec<Vec<Vec<Vec<i64>>>>>>, x: [usize; 6]) {
    let [b, c, d, e, f, g] = x;
    let mut v = a[b][c][d][e][f][g];
    let mut y = [None, None, None, None, None, None];
    for bits in 1_usize..(1 << 6) {
        for i in 0..6 {
            if bits >> i & 1 == 0 {
                y[i] = Some(x[i]);
            } else {
                y[i] = x[i].checked_sub(1);
            }
        }
        match (y[0], y[1], y[2], y[3], y[4], y[5]) {
            (Some(bb), Some(cc), Some(dd), Some(ee), Some(ff), Some(gg)) => {
                let sign = if bits.count_ones() % 2 == 1 { 1 } else { -1 };
                v += sign * a[bb][cc][dd][ee][ff][gg];
            }
            _ => continue,
        }
    }
    a[b][c][d][e][f][g] = v;
}
