use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut ab: [(f64, f64); n],
        mut cd: [(f64, f64); m],
    };

    let mut ub = 1e6;
    let mut lb = 0.0;
    for _ in 0..100 {
        let f = (ub + lb) / 2.0;
        ab.sort_by(|(a, b), (aa, bb)| (b - f * a).partial_cmp(&(bb - f * aa)).unwrap());
        ab.reverse();
        cd.sort_by(|(c, d), (cc, dd)| (d - f * c).partial_cmp(&(dd - f * cc)).unwrap());
        cd.reverse();
        let a = ab[0..5].iter().map(|(a, _)| a).sum::<f64>();
        let b = ab[0..5].iter().map(|(_, b)| b).sum::<f64>();
        let aa = ab[0..4].iter().map(|(a, _)| a).sum::<f64>() + cd[0].0;
        let bb = ab[0..4].iter().map(|(_, b)| b).sum::<f64>() + cd[0].1;
        if (b > 0.0 && b / a >= f) || (bb > 0.0 && bb / aa >= f) {
            lb = f;
        } else {
            ub = f;
        }
    }
    println!("{}", lb);
}
