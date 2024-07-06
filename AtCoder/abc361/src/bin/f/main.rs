use proconio::input;

fn main() {
    input! {
        n: u64,
    };

    let count_squares = {
        let mut ok = 1;
        let mut ng = n + 1;
        while ng - ok > 1 {
            let mid = (ok + ng) / 2;
            if mid.saturating_pow(2) <= n {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok
    };

    let mut values = Vec::new();
    for b in 3..=60 {
        for a in 1_u64.. {
            match a.checked_pow(b) {
                Some(v) if v <= n => {
                    values.push(v);
                }
                _ => break,
            }
        }
    }
    values.sort();
    values.dedup();
    values.retain(|&x| !is_square(x));

    let ans = count_squares + values.len() as u64;
    println!("{}", ans);
}

fn is_square(x: u64) -> bool {
    let y = (x as f64).sqrt() as u64;
    y * y == x
}
