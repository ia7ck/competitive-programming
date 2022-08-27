use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    // http://www7b.biglobe.ne.jp/~math-tota/suB/saikorogame.htm

    let mut ans = 3.5_f64;
    for _ in 2..=n {
        let k = ans.floor() as usize;
        let s = ((k + 1)..=6).sum::<usize>();
        let p = k as f64 / 6.0;
        ans = ans * p + (s as f64 / (6 - k) as f64) as f64 * (1.0 - p);
    }
    println!("{}", ans);
}
