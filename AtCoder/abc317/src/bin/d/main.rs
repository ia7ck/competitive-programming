use proconio::input;

macro_rules! chmin {
    ($a: expr, $b: expr) => {
        $a = $a.min($b);
    };
}

fn main() {
    input! {
        n: usize,
        xyz: [(u64, u64, usize); n],
    };

    let z_sum = xyz.iter().map(|&(_, _, z)| z).sum::<usize>();

    const INF: u64 = u64::MAX / 2;
    const M: usize = 100_000;
    let mut dp_z = vec![INF; M + 1];
    dp_z[0] = 0;
    for (x, y, z) in xyz {
        for k in (0..=M).rev() {
            if dp_z[k] == INF {
                continue;
            }
            if x > y {
                chmin!(dp_z[(k + z).min(M)], dp_z[k]);
            } else {
                let a = (y - x + 1) / 2;
                chmin!(dp_z[(k + z).min(M)], dp_z[k] + a);
            }
        }
    }

    let mut ans = INF;
    for z in ((z_sum + 1) / 2)..=M {
        chmin!(ans, dp_z[z]);
    }
    assert_ne!(ans, INF);
    println!("{}", ans);
}
