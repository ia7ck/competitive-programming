use proconio::input;

fn main() {
    input! {
        n: u64,
        m: u64,
    };

    const M: u64 = 998244353;
    let mut same = m;
    let mut differ = 0;

    for _ in 1..n {
        let same_ = differ;
        let differ_ = same * (m - 1) % M + differ * (m - 2) % M;
        same = same_;
        differ = differ_;
    }
    println!("{}", differ % M);
}
