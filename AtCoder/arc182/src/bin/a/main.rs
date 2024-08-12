use proconio::{input, marker::Usize1};

fn main() {
    input! {
        _n: usize,
        q: usize,
        pv: [(Usize1, u32); q],
    };

    let mut left = vec![false; q];
    let mut right = vec![false; q];
    for (i, &(p, v)) in pv.iter().enumerate() {
        for j in (i + 1)..q {
            let (pp, vv) = pv[j];
            if v <= vv {
                // noop
            } else if p == pp {
                println!("0");
                return;
            } else if p < pp {
                left[i] = true;
                right[j] = true;
            } else {
                right[i] = true;
                left[j] = true;
            }
        }
    }
    let mut ans = 1_u64;
    for i in 0..q {
        if left[i] && right[i] {
            println!("0");
            return;
        }
        if !left[i] && !right[i] {
            ans *= 2;
            ans %= 998244353;
        }
    }
    println!("{}", ans);
}
