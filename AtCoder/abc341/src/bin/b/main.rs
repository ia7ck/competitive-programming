use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u64; n],
        st: [(u64, u64); n - 1],
    };

    for (i, &(s, t)) in st.iter().enumerate() {
        let q = a[i] / s;
        a[i] -= q * s;
        a[i + 1] += q * t;
    }
    let ans = a[n - 1];
    println!("{}", ans);
}
