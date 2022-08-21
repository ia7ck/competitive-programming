use proconio::input;

fn main() {
    input! {
        n: usize,
        mut m: u64,
        k: usize,
        mut a: [u64; n],
    };

    let mut ans = 0_u64;
    for i in (0..32).rev() {
        let mut b: Vec<(u64, usize)> = a
            .iter()
            .copied()
            .enumerate()
            .filter(|&(_, x)| ans & x == ans)
            .map(|(j, x)| {
                let cost = if x >> i & 1 == 1 {
                    0
                } else {
                    (1 << i) - (x & ((1 << i) - 1))
                };
                (cost, j)
            })
            .collect();
        if b.len() < k {
            break;
        }
        b.sort();

        let b = b[..k].to_vec();
        let cost = b.iter().map(|(c, _)| c).sum::<u64>();
        if cost > m {
            continue;
        }
        m -= cost;
        for (c, j) in b {
            a[j] += c;
        }
        ans ^= 1 << i;
    }

    println!("{}", ans);
}
