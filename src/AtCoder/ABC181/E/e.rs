fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let m: usize = rd.get();
    let mut h: Vec<u64> = (0..n).map(|_| rd.get()).collect();
    let w: Vec<u64> = (0..m).map(|_| rd.get()).collect();

    h.sort();
    let mut a = vec![0; n];
    for i in 0..n {
        if i % 2 == 1 {
            a[i] = h[i] - h[i - 1];
            if i >= 2 {
                a[i] += a[i - 2];
            }
        }
    }
    let mut b = vec![0; n];
    for i in (2..n).rev() {
        if i % 2 == 0 {
            b[i] = h[i] - h[i - 1];
            if i + 2 < n {
                b[i] += b[i + 2];
            }
        }
    }
    // println!("{:?}", a);
    // println!("{:?}", b);
    let mut ans = std::u64::MAX;
    for w in w {
        let i = h.lower_bound(&w);
        let mut s = 0;
        if i % 2 == 0 {
            s += h[i] - w;
            if i >= 1 {
                s += a[i - 1];
            }
            if i + 2 < n {
                s += b[i + 2];
            }
        } else {
            s += w - h[i - 1];
            if i >= 2 {
                s += a[i - 2];
            }
            if i + 1 < n {
                s += b[i + 1];
            }
        }
        ans = std::cmp::min(ans, s);
    }
    println!("{}", ans);
}

use std::ops::Range;
/// ソート済の列に対して二分法で"境目"を探します。
pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
    fn split_by(&self, x: &T) -> (Range<usize>, Range<usize>, Range<usize>);
}

impl<T: Ord> BinarySearch<T> for [T] {
    /// ソートされた列 `a` の中で `x` **以上**である最初の要素の位置を返します。全ての要素が `x` 未満のときは `a.len()` を返します。
    ///
    /// # Examples
    /// ```
    /// use binary_search::BinarySearch;
    /// let a = vec![1, 2, 2, 3];
    /// assert_eq!(a.lower_bound(&2), 1);
    /// assert_eq!(a.lower_bound(&9), a.len());
    /// ```
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

    /// ソートされた列 `a` の中で `x` **より大きい**最初の要素の位置を返します。全ての要素が `x` 以下のときは `a.len()` を返します。
    ///
    /// # Examples
    /// ```
    /// use binary_search::BinarySearch;
    /// let a = vec![1, 2, 2, 3];
    /// assert_eq!(a.upper_bound(&2), 3);
    /// assert_eq!(a.upper_bound(&3), a.len());
    /// assert_eq!(a.upper_bound(&9), a.len());
    /// ```
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

    /// ソートされた列 `a` を
    ///
    /// - `x` 未満
    /// - `x` と等しい
    /// - `x` より大きい
    ///
    /// に分ける添字の範囲を tuple で返します。
    ///
    /// # Examples
    /// ```
    /// use binary_search::BinarySearch;
    /// let a = vec![1, 2, 2, 3];
    /// assert_eq!(a.split_by(&0), (0..0, 0..0, 0..a.len()));
    /// assert_eq!(a.split_by(&2), (0..1, 1..3, 3..a.len()));
    /// assert_eq!(a.split_by(&9), (0..a.len(), a.len()..a.len(), a.len()..a.len()));
    /// ```
    fn split_by(&self, x: &T) -> (Range<usize>, Range<usize>, Range<usize>) {
        let i = self.lower_bound(x);
        let j = self.upper_bound(x);
        (0..i, i..j, j..self.len())
    }
}

pub struct ProconReader<R: std::io::Read> {
    reader: R,
}

impl<R: std::io::Read> ProconReader<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }
    pub fn get<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .reader
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&byte| byte == b' ' || byte == b'\n' || byte == b'\r')
            .take_while(|&byte| byte != b' ' && byte != b'\n' && byte != b'\r')
            .collect::<Vec<_>>();
        std::str::from_utf8(&buf)
            .unwrap()
            .parse()
            .ok()
            .expect("Parse Error.")
    }
}
