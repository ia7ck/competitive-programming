use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        _k: usize,
        a: [[u8; n]; n],
        q: usize,
        queries: [(Usize1, Usize1); q],
    };

    const INF: u64 = std::u64::MAX / 2;
    let mut d = vec![vec![INF; n]; n];
    for v in 0..n {
        d[v][v] = 0;
    }
    for s in 0..n {
        for t in 0..n {
            if a[s][t] == 1 {
                d[s][t] = 1;
            }
        }
    }
    for v in 0..n {
        for s in 0..n {
            for t in 0..n {
                d[s][t] = d[s][t].min(d[s][v] + d[v][t]);
            }
        }
    }

    for (s, t) in queries {
        let s = s % n;
        let t = t % n;
        if a[s][t] == 1 && a[s][t] == 1 {
            println!("1");
            continue;
        }
        let mut ans = INF;
        for v in 0..n {
            if a[s][v] == 1 {
                ans = ans.min(1 + d[v][t]);
            }
        }
        if ans == INF {
            println!("-1");
        } else {
            println!("{}", ans);
        }
    }
}
