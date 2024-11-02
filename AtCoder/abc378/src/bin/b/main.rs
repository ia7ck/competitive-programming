use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        qr: [(usize, usize); n],
        q: usize,
        td: [(Usize1, usize); q],
    };

    for (t, d) in td {
        let (q, r) = qr[t];
        let p = d % q;
        let plus = if p <= r {
            r - p
        } else {
            (q - p) + r
        };
        let ans = d + plus;
        println!("{}", ans);
    }
}
