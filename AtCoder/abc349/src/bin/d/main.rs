use proconio::input;

fn main() {
    input! {
        l: u64,
        r: u64,
    };

    let mut ans = solve(l, r);
    ans.sort();
    println!("{}", ans.len());
    for (l, r) in ans {
        println!("{} {}", l, r);
    }
}

fn solve(l: u64, r: u64) -> Vec<(u64, u64)> {
    assert!(l <= r);
    if l == r {
        return Vec::new();
    }
    if (r - l).is_power_of_two() {
        if l % (r - l) == 0 {
            return vec![(l, r)];
        }
    }
    let mut k = (r - l).next_power_of_two() / 2;
    while k >= 1 {
        let mut best = Option::<Vec<(u64, u64)>>::None;
        let start = (l + k - 1) / k * k;
        for ll in (start..r).step_by(k as usize) {
            if ll + k <= r {
                let mut res = Vec::new();
                res.extend(solve(l, ll));
                res.push((ll, ll + k));
                res.extend(solve(ll + k, r));
                let new_best = match best.take() {
                    None => res,
                    Some(best) if best.len() > res.len() => res,
                    Some(best) => best,
                };
                best = Some(new_best);
            }
        }
        if let Some(best) = best {
            return best;
        }
        k /= 2;
    }
    unreachable!()
}
