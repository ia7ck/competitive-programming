use scanner_proc_macro::insert_scanner;

#[insert_scanner]
fn main() {
    let (n, mut k, x) = scan!((usize, u64, u64));
    let mut a = scan!(u64; n);

    for i in 0..n {
        let l = k.min(a[i] / x);
        a[i] -= l * x;
        k -= l;
    }

    if k >= 1 {
        let less = a.iter().all(|&a| a < x);
        assert!(less);
        a.sort();
        a.reverse();
        for i in 0..n {
            if k >= 1 {
                a[i] = 0;
                k -= 1;
            }
        }
    }

    let ans = a.iter().sum::<u64>();
    println!("{}", ans);
}
