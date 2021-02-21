use std::io::Read;

fn read<T: std::str::FromStr>() -> T {
    let token: String = std::io::stdin()
        .bytes()
        .map(|c| c.ok().unwrap() as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().unwrap()
}

mod binary_search {
    use std::ops::Range;
    pub trait BinarySearch<T> {
        fn lower_bound(&self, x: &T) -> usize;
        fn upper_bound(&self, x: &T) -> usize;
        fn split_by(&self, x: &T) -> (Range<usize>, Range<usize>, Range<usize>);
    }

    // min index self[i] >= x
    // that is, any j (j < i) holds self[j] < x
    impl<T: Ord> BinarySearch<T> for [T] {
        fn lower_bound(&self, x: &T) -> usize {
            if self[0] >= *x {
                return 0;
            }
            let mut lf = 0;
            let mut rg = self.len();
            // self[lf] < x
            while rg - lf > 1 {
                let md = (rg + lf) / 2;
                if self[md] < *x {
                    lf = md;
                } else {
                    rg = md;
                }
            }
            rg
        }

        // min index self[i] > x
        // that is, any j (j < i) holds self[j] <= x
        fn upper_bound(&self, x: &T) -> usize {
            if self[0] > *x {
                return 0;
            }
            let mut lf = 0;
            let mut rg = self.len();
            // self[lf] <= x
            while rg - lf > 1 {
                let md = (rg + lf) / 2;
                if self[md] <= *x {
                    lf = md;
                } else {
                    rg = md;
                }
            }
            rg
        }

        fn split_by(&self, x: &T) -> (Range<usize>, Range<usize>, Range<usize>) {
            let i = self.lower_bound(x);
            let j = self.upper_bound(x);
            (0..i, i..j, j..self.len())
        }
    }
}

pub struct UnionFind {
    par: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        UnionFind {
            par: (0..n).map(|i| i).collect::<Vec<_>>(),
            size: vec![1; n],
        }
    }
    pub fn find(&mut self, i: usize) -> usize {
        if self.par[i] == i {
            self.par[i]
        } else {
            self.par[i] = self.find(self.par[i]);
            self.par[i]
        }
    }
    pub fn unite(&mut self, i: usize, j: usize) {
        let i = self.find(i);
        let j = self.find(j);
        if i == j {
            return;
        }
        let (i, j) = if self.size[i] >= self.size[j] {
            (i, j)
        } else {
            (j, i)
        };
        self.par[j] = i;
        self.size[i] += self.size[j];
    }
    pub fn get_size(&mut self, i: usize) -> usize {
        let p = self.find(i);
        self.size[p]
    }
}

use binary_search::BinarySearch;

fn main() {
    let n: usize = read();
    let a: i64 = read();
    let b: i64 = read();
    let x: Vec<i64> = (0..n).map(|_| read()).collect::<Vec<_>>();

    let mut uf = UnionFind::new(n);
    let mut c = vec![0; n + 1];
    for (i, &y) in x.iter().enumerate() {
        let j = x.lower_bound(&(y + a));
        let k = x.upper_bound(&(y + b));
        if j < k {
            uf.unite(i, j);
            c[j] += 1;
            c[k - 1] -= 1;
        }
    }
    for i in 0..n {
        c[i + 1] += c[i];
    }
    for i in 0..(n - 1) {
        if c[i] > 0 {
            uf.unite(i, i + 1);
        }
    }
    for i in 0..n {
        println!("{}", uf.get_size(i));
    }
}
