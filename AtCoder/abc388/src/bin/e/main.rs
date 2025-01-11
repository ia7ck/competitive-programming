use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    };

    let mut ok = 0;
    let mut ng = n / 2 + 1;
    while ng - ok > 1 {
        let k = (ng + ok) / 2;
        if f(&a[..k], &a[k..]) >= k {
            ok = k;
        } else {
            ng = k;
        }
    }
    println!("{}", ok);
}

fn f(low: &[u32], high: &[u32]) -> usize {
    let mut j = 0;
    for (i, &x) in low.iter().enumerate() {
        while j < high.len() && x > high[j] / 2 {
            j += 1;
        }
        if j >= high.len() {
            return i;
        }
        assert!(x <= high[j] / 2);
        j += 1;
    }
    low.len()
}
