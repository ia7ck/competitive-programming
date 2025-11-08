use proconio::{input, marker::Usize1};

fn main() {
    input! {
        x: u64,
        n: usize,
        w: [u64; n],
        q: usize,
        queries: [Usize1; q],
    };

    let mut ans = x;
    let mut on = vec![false; n];
    for p in queries {
        if on[p] {
            ans -= w[p];
        } else {
            ans += w[p];
        }
        println!("{}", ans);

        on[p] = !on[p];
    }
}
