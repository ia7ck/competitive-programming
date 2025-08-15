use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i32, i32); n],
    };

    // 傾きで線分を分類
    let mut segments = HashMap::new();
    for i in 0..n {
        for j in 0..i {
            let (xi, yi) = xy[i];
            let (xj, yj) = xy[j];
            let mut dy = yj - yi;
            let mut dx = xj - xi;
            let g = gcd(dy, dx);
            assert_ne!(g, 0);
            dy /= g;
            dx /= g;
            // dy/dx
            if dy.is_negative() {
                dy *= -1;
                dx *= -1;
            }
            if dy == 0 {
                assert_ne!(dx, 0);
                dx = dx.signum();
            }
            if dx == 0 {
                assert_ne!(dy, 0);
                dy = dy.signum();
            }
            segments.entry((dy, dx)).or_insert(Vec::new()).push((i, j));
        }
    }

    let mut ans = 0_usize;
    for v in segments.values() {
        ans += v.len() * (v.len() - 1) / 2;
    }

    // 平行四辺形を引く
    let mut sub = 0;
    for v in segments.into_values() {
        // 長さで線分を分類、傾きは全て同じ
        let mut seg = HashMap::new();
        for (i, j) in v {
            let (xi, yi) = xy[i];
            let (xj, yj) = xy[j];
            let dist = xi.abs_diff(xj).pow(2) + yi.abs_diff(yj).pow(2);
            *seg.entry(dist).or_insert(0) += 1;
        }
        for v in seg.into_values() {
            sub += v * (v - 1) / 2;
        }
    }
    assert_eq!(sub % 2, 0);

    ans -= sub / 2;

    println!("{}", ans);
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
