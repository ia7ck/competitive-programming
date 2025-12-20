use proconio::input;

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize,
            wp: [(u64, u64); n],
        };

        solve(n, wp);
    }
}

fn solve(_n: usize, wp: Vec<(u64, u64)>) {
    let mut wp = wp;
    wp.sort_unstable_by_key(|&(w, p)| w + p);

    let mut w_sum = 0;
    let mut p_sum = wp.iter().map(|&(_, p)| p).sum::<u64>();
    let mut ans = 0;
    for (w, p) in wp {
        if w_sum + w <= p_sum - p {
            w_sum += w;
            p_sum -= p;
            ans += 1;
        }
    }
    println!("{}", ans);
}
