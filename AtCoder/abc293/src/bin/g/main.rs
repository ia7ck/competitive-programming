use floor_sqrt::floor_sqrt;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        lr: [(Usize1, usize); q],
    };

    let b = 1.max(n / floor_sqrt(q as u64) as usize);
    let mut lri = lr.into_iter().enumerate().collect::<Vec<_>>();
    lri.sort_by_key(|&(_, (l, r))| (l / b, r));
    let (mut l, mut r) = (0, 0);
    let mut f = [0_u64; 200001];
    let mut c = 0;
    let mut ans = vec![0; q];
    for (i, (ll, rr)) in lri {
        // ひろげる
        while ll < l {
            l -= 1;
            c += f[a[l]] * f[a[l]].saturating_sub(1) / 2;
            f[a[l]] += 1;
        }
        while r < rr {
            c += f[a[r]] * f[a[r]].saturating_sub(1) / 2;
            f[a[r]] += 1;
            r += 1;
        }

        // ちぢめる
        while l < ll {
            f[a[l]] -= 1;
            c -= f[a[l]] * f[a[l]].saturating_sub(1) / 2;
            l += 1;
        }
        while rr < r {
            r -= 1;
            f[a[r]] -= 1;
            c -= f[a[r]] * f[a[r]].saturating_sub(1) / 2;
        }

        ans[i] = c;
    }
    for i in 0..q {
        println!("{}", ans[i]);
    }
}
