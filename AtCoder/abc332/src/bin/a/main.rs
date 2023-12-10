use proconio::input;

fn main() {
    input! {
        n: usize,
        s: u64,
        k: u64,
        pq: [(u64, u64); n],
    };

    let mut ans = 0;
    for (p, q) in pq {
        ans += p * q;
    }
    if ans < s {
        ans += k;
    }
    println!("{}", ans);
}
