use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        tp: [(i64, Usize1); m],
        q: usize,
        ab: [(Usize1, Usize1); q],
    };

    // threshold
    let l = f64::sqrt(n as f64) as usize;

    let ranges = {
        let mut ranges = vec![vec![]; n];
        let mut enter_t = vec![Option::<i64>::None; n];
        for &(t, p) in &tp {
            enter_t[p] = match enter_t[p].take() {
                None => Some(t),
                Some(s) => {
                    ranges[p].push((s, t));
                    None
                }
            };
        }
        ranges
    };

    let ans_large = {
        let mut ans = vec![vec![]; n];
        for i in 0..n {
            if ranges[i].len() < l {
                continue;
            }
            ans[i] = vec![0; n];
            let mut enter_t_i = Option::<i64>::None;
            // enter_sum[i] := undefined
            let mut enter_sum = vec![Option::<i64>::None; n];
            for &(t, p) in &tp {
                if i == p {
                    enter_t_i = match enter_t_i.take() {
                        None => Some(t),
                        Some(s) => {
                            ans[i][i] += t - s;
                            None
                        }
                    };
                } else {
                    // range i : [ ... ]
                    // range p : ( ... )
                    match (enter_t_i, enter_sum[p]) {
                        // (--[---)---]    or    (--[---]---)
                        // ^                     ^
                        (None, None) => {
                            enter_sum[p] = Some(ans[i][i]);
                        }
                        // [--(---]---)    or    (--[---]---)
                        //            ^                     ^
                        (None, Some(v)) => {
                            ans[i][p] += ans[i][i] - v;
                            enter_sum[p] = None;
                        }
                        // [--(---]---)    or    [--(---)---]
                        //    ^                     ^
                        (Some(s), None) => {
                            enter_sum[p] = Some(ans[i][i] + (t - s));
                        }
                        // (--[---)---]    or    [--(---)---]
                        //        ^                     ^
                        (Some(s), Some(v)) => {
                            ans[i][p] += ans[i][i] + (t - s) - v;
                            enter_sum[p] = None;
                        }
                    }
                }
            }
        }
        ans
    };

    for (a, b) in ab {
        if ranges[a].len() < l && ranges[b].len() < l {
            let mut i = 0;
            let mut ans = 0;
            for &(s, t) in &ranges[a] {
                if let Some(&(ss, tt)) = ranges[b].get(i) {
                    ans += (tt.min(t) - ss.max(s)).max(0);
                }
                while i + 1 < ranges[b].len() && ranges[b][i + 1].0 <= t {
                    i += 1;
                    let (ss, tt) = ranges[b][i];
                    ans += (tt.min(t) - ss.max(s)).max(0);
                }
            }
            println!("{}", ans);
        } else {
            let ans = if ranges[a].len() >= l {
                assert_eq!(ans_large[a].len(), n);
                ans_large[a][b]
            } else {
                assert_eq!(ans_large[b].len(), n);
                ans_large[b][a]
            };
            println!("{}", ans);
        }
    }
}
