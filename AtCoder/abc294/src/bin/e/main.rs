use proconio::input;

fn main() {
    input! {
        l: usize,
        n1: usize,
        n2: usize,
        vl1: [(u32, usize); n1],
        vl2: [(u32, usize); n2],
    };

    let (mut i1, mut p1) = (0, 0);
    let (mut i2, mut p2) = (0, 0);
    let mut ans = 0;
    while i1 < n1 || i2 < n2 {
        // eprintln!("i1 = {}, p1 = {}, i2 = {}, p2 = {}", i1, p1, i2, p2);
        if p1 < p2 {
            assert!(i1 < n1);
            assert!(p1 < l);
            let (v1, l1) = vl1[i1];
            let (v2, _) = vl2[i2 - 1];
            if v1 == v2 {
                let x = if p1 + l1 <= p2 { l1 } else { p2 - p1 };
                ans += x;
            }
            i1 += 1;
            p1 += l1;
        } else if p1 > p2 {
            assert!(i2 < n2);
            assert!(p2 < l);
            let (v2, l2) = vl2[i2];
            let (v1, _) = vl1[i1 - 1];
            if v1 == v2 {
                let x = if p2 + l2 <= p1 { l2 } else { p1 - p2 };
                ans += x;
            }
            i2 += 1;
            p2 += l2;
        } else {
            assert!(i1 < n1);
            assert!(p1 < l);
            assert!(i2 < n2);
            assert!(p2 < l);
            let (v1, l1) = vl1[i1];
            let (v2, l2) = vl2[i2];
            if v1 == v2 {
                ans += l1.min(l2);
            }
            i1 += 1;
            p1 += l1;
            i2 += 1;
            p2 += l2;
        }
    }

    println!("{}", ans);
}
