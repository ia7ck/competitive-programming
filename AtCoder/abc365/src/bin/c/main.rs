use proconio::input;

fn main() {
    input! {
        n: usize,
        m: u64,
        a: [u64; n],
    };

    if a.iter().sum::<u64>() <= m {
        println!("infinite");
        return;
    }

    let mut ok = 0;
    let mut ng = m + 1;
    while ng - ok > 1 {
        let mid = (ng + ok) / 2;
        let cost = a.iter().map(|&x| x.min(mid)).sum::<u64>();
        if cost <= m {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
