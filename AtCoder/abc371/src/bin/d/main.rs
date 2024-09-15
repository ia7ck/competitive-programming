use proconio::input;

use zarts::SortedSeq;

fn main() {
    input! {
        n: usize,
        xs: [i64; n],
        ps: [u64; n],
        q: usize,
        lr: [(i64, i64); q],
    };

    let mut values = Vec::new();
    for &x in &xs {
        values.push(x);
    }
    for &(l, r) in &lr {
        values.push(l);
        values.push(r);
    }
    let seq = SortedSeq::from_iter(values);

    let mut cum_sum = vec![0; seq.size()];
    for i in 0..n {
        cum_sum[seq.ord(&xs[i])] += ps[i];
    }
    for i in 1..cum_sum.len() {
        cum_sum[i] += cum_sum[i - 1];
    }

    for &(l, r) in &lr {
        let (l, r) = (seq.ord(&l), seq.ord(&r));
        let mut ans = cum_sum[r];
        if l >= 1 {
            ans -= cum_sum[l - 1];
        }
        println!("{}", ans);
    }
}
