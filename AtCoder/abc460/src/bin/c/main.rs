use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [u32; n],
        mut b: [u32; m],
    };

    a.sort_unstable();
    b.sort_unstable();

    let mut i = 0;
    let mut ans = 0;
    for b in b {
        while i < n && b > a[i] * 2 {
            i += 1;
        }
        if i < n {
            assert!(b <= a[i] * 2);
            ans += 1;
            i += 1;
        }
    }

    println!("{}", ans);
}
