use proconio::input;
use run_length::RunLength;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };

    let mut ans = n; // l = r = i
    let diffs = a.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
    for (_, len) in RunLength::new(diffs.iter()) {
        ans += len * (len + 1) / 2;
    }

    println!("{}", ans);
}
