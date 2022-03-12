use scanner_proc_macro::insert_scanner;

#[insert_scanner]
fn main() {
    let t = scan!(usize);
    for _ in 0..t {
        let n = scan!(usize);
        let a = scan!(u8; n);
        solve(n, a);
    }
}

fn solve(_n: usize, a: Vec<u8>) {
    let ans = if let Some(l) = a.iter().position(|&x| x == 0) {
        let r = a.iter().rposition(|&x| x == 0).unwrap();
        assert_eq!(a[l - 1], 1);
        assert_eq!(a[r + 1], 1);
        (r + 1) - (l - 1)
    } else {
        0
    };
    println!("{}", ans);
}