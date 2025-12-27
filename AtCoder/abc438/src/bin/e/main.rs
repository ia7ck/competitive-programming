use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [Usize1; n],
        queries: [(usize, Usize1); q],
    };

    #[derive(Debug, Clone, Copy)]
    struct S {
        pos: usize,
        total: usize,
    }

    // doubling
    let mut dp = {
        let v = a
            .iter()
            .enumerate()
            .map(|(i, &a)| S {
                pos: a,
                total: i + 1,
            })
            .collect::<Vec<_>>();
        vec![v]
    };
    for i in 1..32 {
        let mut v = Vec::new();
        for p in 0..n {
            let s = dp[i - 1][p];
            let t = dp[i - 1][s.pos];
            v.push(S {
                pos: t.pos,
                total: s.total + t.total,
            });
        }
        dp.push(v);
    }

    for (t, b) in queries {
        let mut b = b;
        let mut ans = 0;
        for i in 0..32 {
            if t >> i & 1 == 1 {
                let s = dp[i][b];
                b = s.pos;
                ans += s.total;
            }
        }
        println!("{}", ans);
    }
}
