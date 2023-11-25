use proconio::input;

fn main() {
    input! {
        d: i64,
    };

    let ys = Vec::from_iter(0..2_000_000_i64);
    let mut ans = d;
    for x in 0..2_000_000_i64 {
        let i = ys.partition_point(|&y| x.pow(2) + y.pow(2) - d < 0);
        if i >= 1 {
            ans = ans.min(-(x.pow(2) + ys[i - 1].pow(2) - d));
        }
        if i < ys.len() {
            ans = ans.min(x.pow(2) + ys[i].pow(2) - d);
        }
    }
    println!("{}", ans);
}
