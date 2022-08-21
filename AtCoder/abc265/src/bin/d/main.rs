use proconio::input;

fn main() {
    input! {
        n: usize,
        p: u64,
        q: u64,
        r: u64,
        a: [u64; n],
    };

    let mut cum = vec![0; n + 1];
    for i in 0..n {
        cum[i + 1] = cum[i] + a[i];
    }

    for w in 1..=n {
        let s = cum[w];
        if s >= r && cum.binary_search(&(s - r)).is_ok() {
            if s - r >= q && cum.binary_search(&(s - r - q)).is_ok() {
                if s - r - q >= p && cum.binary_search(&(s - r - q - p)).is_ok() {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
}
