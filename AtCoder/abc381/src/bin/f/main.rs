use proconio::{input, marker::Usize1};

macro_rules! chmin {
    ($a: expr, $b: expr) => {
        $a = $a.min($b);
    };
}

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    };

    let mut next = vec![vec![Option::<usize>::None; 20]; n];
    for i in (0..(n - 1)).rev() {
        next[i] = next[i + 1].clone();
        next[i][a[i + 1]] = Some(i + 1);
    }

    const INF: usize = usize::MAX;
    // dp[S] := Sの要素のみを使えるとき(a[0], a[1], ..., a[r])の部分列に長さ2|S|の1122数列があるような最小のr
    let mut dp = vec![INF; 1 << 20];
    for x in 0..20 {
        if a[0] == x {
            dp[1 << x] = next[0][x].unwrap_or(INF);
        } else {
            dp[1 << x] = next[0][x].and_then(|p| next[p][x]).unwrap_or(INF);
        }
    }
    for s in 1..(1 << 20) {
        if dp[s] == usize::MAX {
            continue;
        }
        for x in 0..20 {
            if s >> x & 1 == 0 {
                let p = next[dp[s]][x].and_then(|p| next[p][x]).unwrap_or(INF);
                if p < n {
                    chmin!(dp[s ^ (1 << x)], p);
                }
            }
        }
    }

    let mut ans = 0;
    for s in 1..(1 << 20) {
        if dp[s] < n {
            ans = ans.max(s.count_ones() * 2);
        }
    }

    println!("{}", ans);
}
