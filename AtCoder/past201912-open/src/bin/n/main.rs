use proconio::input;

fn main() {
    input! {
        n: usize,
        w: u64,
        c: u64,
        mut lrp: [(u64, u64, u64); n],
    };

    lrp.push((0, 0, 0));
    lrp.push((w, w, 0));
    let mut lrp_order_by_l = lrp.clone();
    let mut lrp_order_by_r = lrp.clone();
    lrp_order_by_l.sort_by_key(|&(l, r, p)| (l, r, p));
    lrp_order_by_r.sort_by_key(|&(l, r, p)| (r, l, p));

    let mut cum_sum_front = vec![0; n + 2];
    let mut cum_sum_back = vec![0; n + 2];
    for i in 1..(n + 2) {
        let (_, _, p) = lrp_order_by_r[i];
        cum_sum_front[i] = cum_sum_front[i - 1] + p;
    }
    for i in (0..(n + 1)).rev() {
        let (_, _, p) = lrp_order_by_l[i];
        cum_sum_back[i] = cum_sum_back[i + 1] + p;
    }

    let mut total = 0;
    for &(_, _, p) in &lrp {
        total += p;
    }
    let mut ans = total;
    for (_, r, _) in lrp {
        let l = r + c;
        if l > w {
            continue;
        }
        // 0 〜 r, l 〜 w の石はそのまま

        // lrp_order_by_r[ok].1 <= r
        let mut ok = 0;
        let mut ng = n + 2;
        while ng - ok > 1 {
            let mid = (ok + ng) / 2;
            let (_, rr, _) = lrp_order_by_r[mid];
            if rr <= r {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        let front = cum_sum_front[ok];

        // lrp_order_by_l[ok].0 >= l
        let mut ng = 0;
        let mut ok = n + 2;
        while ok - ng > 1 {
            let mid = (ok + ng) / 2;
            let (ll, _, _) = lrp_order_by_l[mid];
            if ll >= l {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        let back = cum_sum_back[ok];

        ans = ans.min(total - front - back);
    }

    println!("{}", ans);
}
