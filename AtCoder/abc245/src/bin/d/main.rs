use join::Join;
use scanner_proc_macro::insert_scanner;

#[insert_scanner]
fn main() {
    let (n, m) = scan!((usize, usize));
    let mut a = scan!(i64; n + 1);
    let mut c = scan!(i64; n + m + 1);

    a.reverse();
    c.reverse();

    let mut b = Vec::new();
    for i in 0..=m {
        assert_eq!(c[i] % a[0], 0);
        let d = c[i] / a[0];
        b.push(d);
        if d != 0 && i + n <= m + n {
            for j in 0..=n {
                c[i + j] -= a[j] * d;
            }
        }
    }

    for c in c {
        assert_eq!(c, 0);
    }

    b.reverse();
    assert_ne!(b[m], 0);
    println!("{}", b.iter().join(" "));
}
