use scanner_proc_macro::insert_scanner;

#[insert_scanner]
fn main() {
    let (n, k) = scan!((usize, usize));
    let a = scan!(usize; n);

    const B: usize = 40;
    let mut count = vec![vec![0; n]; B];
    for i in 0..n {
        count[0][i] = a[i];
    }
    for b in 0..(B - 1) {
        for i in 0..n {
            count[b + 1][i] = count[b][i] + count[b][(i + count[b][i]) % n];
        }
    }
    let mut ans = 0;
    for b in 0..B {
        if k >> b & 1 == 1 {
            ans += count[b][ans % n];
        }
    }
    println!("{}", ans);
}
