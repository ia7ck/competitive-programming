pub mod fenwick_tree {
    pub struct FenwickTree {
        n: usize,
        dat: Vec<i64>,
    }

    impl FenwickTree {
        pub fn new(n: usize) -> FenwickTree {
            let n2 = n.next_power_of_two();
            FenwickTree {
                n: n2,
                dat: vec![0; n2 + 1],
            }
        }

        pub fn add(&mut self, k: usize, v: i64) {
            let mut i = k as i32;
            while i <= self.n as i32 {
                self.dat[i as usize] += v;
                i += i & (-i);
            }
        }

        pub fn sum(&self, l: usize, r: usize) -> i64 {
            self._sum(r) - self._sum(l - 1)
        }

        pub fn _sum(&self, r: usize) -> i64 {
            let mut result = 0;
            let mut i = r as i32;
            while i >= 1 {
                result += self.dat[i as usize];
                i -= i & (-i);
            }
            result
        }
    }
}

extern crate proconio;
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        lr: [(usize, usize); q],
    }
    let mut lri = lr
        .iter()
        .zip(0..q)
        .map(|(&(l, r), i)| (l, r, i as usize))
        .collect::<Vec<_>>();
    lri.sort_by(|x, y| x.1.cmp(&y.1));
    let mut t = fenwick_tree::FenwickTree::new(n);
    let mut last: Vec<Option<usize>> = vec![None; n + 1];
    let mut j = 0 as usize;
    let mut ans = vec![-1; q];
    for i in 0..n {
        if let Some(k) = last[a[i]] {
            t.add(k, -1);
        }
        last[a[i]] = Some(i + 1);
        t.add(i + 1, 1);
        while j < q && lri[j].1 == i + 1 {
            let (l, _, k) = lri[j];
            ans[k] = t.sum(l, i + 1);
            j += 1;
        }
    }
    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
}
