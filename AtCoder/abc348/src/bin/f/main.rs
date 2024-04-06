use fixedbitset::FixedBitSet;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[usize; m]; n],
    };

    // bitset[i][j] := #{ k | a[i][k] == a[j][k] } % 2
    let mut bitset = vec![FixedBitSet::with_capacity(n); n];
    let mut tmp = vec![FixedBitSet::with_capacity(n); 1000];
    for k in 0..m {
        for i in 0..n {
            tmp[a[i][k]].set(i, true);
        }

        for i in 0..n {
            bitset[i] ^= &tmp[a[i][k]];
        }

        for i in 0..n {
            tmp[a[i][k]].set(i, false);
        }
    }

    let mut ans = 0;
    for i in 0..n {
        ans += bitset[i].count_ones((i + 1)..);
    }
    println!("{}", ans);
}
