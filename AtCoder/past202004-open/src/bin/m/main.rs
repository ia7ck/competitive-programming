use proconio::input;

#[allow(unused_macros)]
macro_rules! dbg {
    ($($arg:tt)*) => {{
        #[cfg(debug_assertions)]
        std::dbg!($($arg)*);
    }};
}

fn main() {
    input! {
        d: usize,
        l: usize,
        n: usize,
        c: [usize; d],
    };

    const M: usize = 100_000;

    let mut pos = vec![vec![]; M + 1];
    for i in 0..d {
        pos[c[i]].push(i + 1);
    }
    for i in 0..d {
        pos[c[i]].push(i + 1 + d);
    }
    // 2回連結だと足りないので3回連結しておく
    for i in 0..d {
        pos[c[i]].push(i + 1 + d + d);
    }

    let mut cum_sum = vec![vec![]; M + 1];
    for c in 1..=M {
        cum_sum[c] = vec![0; pos[c].len()];
    }
    for c in 1..=M {
        for j in 1..pos[c].len() {
            cum_sum[c][j] += cum_sum[c][j - 1] + (pos[c][j] - pos[c][j - 1] - 1) / l;
        }
    }

    for _ in 0..n {
        input! {
            k: usize,
            f: usize,
            t: usize,
        };

        let t = t - 1;
        let pos = &pos[k];
        let cum_sum = &cum_sum[k];

        if pos.is_empty() {
            println!("0");
            continue;
        }

        let i = match pos.binary_search(&f) {
            Ok(i) => i,
            Err(i) => {
                assert!(i < pos.len());
                i
            }
        };
        // 最初の好みの料理
        let p = pos[i];
        assert!(p >= f);
        // f+1からp-1までの間に好みじゃない料理を食べる回数
        let normal = (p - f).saturating_sub(1) / l;
        if normal >= t {
            println!("0");
            continue;
        }
        let t = if p > f {
            // f, ..., p
            t - normal - 1
        } else {
            // f = p
            assert_eq!(normal, 0);
            t
        };
        // ループに入る前に好みの料理を食べる回数
        let pre_fav = if p > f && pos.binary_search(&f).is_ok() {
            2 // f, p
        } else {
            1 // p
        };

        // 好み + その他
        let s = pos.len() / 3 + (cum_sum[i + pos.len() / 3] - cum_sum[i]);
        assert!(s >= 1);
        // ループ中に好みの料理を食べる回数
        let loop_fav = t / s * (pos.len() / 3);
        let t = t % s;

        // (j - i) + (cum_sum[j] - cum_sum[i]) <= t となる最大の j
        let mut ok = i;
        let mut ng = cum_sum.len();
        while ng - ok > 1 {
            let mid = (ok + ng) / 2;
            if (mid - i) + (cum_sum[mid] - cum_sum[i]) <= t {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        let j = ok;
        // ループ終了後に好みの料理を食べる回数
        let post_fav = j - i;

        println!("{}", pre_fav + loop_fav + post_fav);
    }
}
