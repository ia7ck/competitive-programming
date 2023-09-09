use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        l: [u64; n],
    };

    let mut small = 0;
    let mut large = u64::MAX / 2;
    while small + 1 < large {
        let mid = (small + large) / 2;
        if l.iter().any(|&l| l > mid) {
            small = mid;
            continue;
        }

        let mut cnt = 1;
        let mut w = l[0];
        for &l in &l[1..] {
            if w + (1 + l) <= mid {
                w += 1 + l;
            } else {
                cnt += 1;
                w = l;
            }
        }
        if cnt <= m {
            large = mid;
        } else {
            small = mid;
        }
    }
    
    println!("{}", large);
}
