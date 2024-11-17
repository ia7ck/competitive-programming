use mod_int::ModInt998244353;
use proconio::{input, marker::Usize1};
use segment_tree::SegmentTree;

type Mint = ModInt998244353;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [Usize1; n],
    };

    let all = {
        let mut inv = 0_usize;
        let mut seg = SegmentTree::new(n, 0, |a, b| a + b);
        for i in 0..n {
            inv += seg.fold(p[i]..);
            seg.set(p[i], 1);
        }
        Mint::from(inv)
    };

    let shuffle = Mint::from(k) * (k - 1) / 2 / 2;

    let mut ans = Mint::new(0);
    let mut inv = 0_usize;
    let mut seg = SegmentTree::new(n, 0, |a, b| a + b);
    for i in 0..n {
        if i >= k {
            inv -= seg.fold(..p[i - k]);
            seg.set(p[i - k], 0);
        }
        inv += seg.fold(p[i]..);
        seg.set(p[i], 1);
        if i >= k - 1 {
            ans += all - Mint::from(inv) + shuffle;
        }
    }
    ans /= n - k + 1;
    println!("{}", ans.val());
}
