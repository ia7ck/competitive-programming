use proconio::input;

use fenwick_tree::FenwickTree;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[u32; m]; n],
    };

    let mut values = Vec::new();
    for a in &a {
        for &x in a {
            values.push(x);
        }
    }
    values.sort();
    let ord = |x: u32| -> usize {
        values.binary_search(&x).unwrap()
    };

    let mut t = FenwickTree::new(n * m + 1, 0);
    let mut ans = 0_usize;
    for i in (0..n).rev() {
        let mut a = a[i].clone();
        a.sort();
        let mut p = 0;
        for (j, &x) in a.iter().enumerate() {
            if i + 1 < n {
                ans += t.sum(0..ord(x));
            }
            p += j + 1;
        }
        ans += p * (n - i - 1);
        for &x in &a {
            t.add(ord(x), 1);
        }
    }
    println!("{}", ans);
}
