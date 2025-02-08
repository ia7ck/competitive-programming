use ac_library::{Additive, Segtree};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
    };

    let mut ans = vec![0; n];
    let mut seg = Segtree::<Additive<usize>>::new(n);
    for i in 0..n {
        seg.set(i, 1);
    }
    for i in (0..n).rev() {
        let pos = seg.max_right(0, |&s| s <= p[i]);
        ans[pos] = i;
        seg.set(pos, 0);
    }

    println!(
        "{}",
        ans.iter()
            .map(|x| (x + 1).to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
