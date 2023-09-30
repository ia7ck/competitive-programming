use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    };

    for i in 1..=n {
        let ans = match a.binary_search(&i) {
            Ok(_) => 0,
            Err(j) => {
                assert!(j < n);
                assert!(a[j] >= i);
                a[j] - i
            }
        };
        println!("{}", ans);
    }
}
