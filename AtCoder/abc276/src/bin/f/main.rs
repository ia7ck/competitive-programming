use fenwick_tree::FenwickTree;
use mod_int::ModInt998244353;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    };

    let mut ai = Vec::new();
    for i in 0..n {
        ai.push((a[i], i));
    }

    ai.sort();
    ai.reverse();
    let mut plus = vec![0; n];
    let mut f = FenwickTree::new(n, 0);
    let mut g = FenwickTree::new(n, 0);
    for (x, i) in ai {
        plus[i] = ((i as u64 - f.sum(0..i)) * x + g.sum(0..i)) * 2 + x;
        f.add(i, 1);
        g.add(i, x);
    }

    let mut sum = 0;
    for i in 0..n {
        sum += plus[i];
        let ans = ModInt998244353::from(sum) / (i + 1).pow(2);
        println!("{}", ans.val());
    }
}
