use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        p: usize,
    };

    let mut d = m;
    let mut ans = 0;
    while d <= n {
        ans += 1;
        d += p;
    }
    println!("{}", ans);
}
