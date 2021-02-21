fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let t: u64 = rd.get();
    let a: Vec<u64> = (0..n).map(|_| rd.get()).collect();

    let b = &a[..(n / 2)];
    let c = &a[(n / 2)..];
    let mut s = (0..(1 << b.len()))
        .map(|bit| {
            (0..b.len())
                .map(|i| if bit >> i & 1 == 0 { 0 } else { b[i] })
                .sum::<u64>()
        })
        .collect::<Vec<_>>();
    s.sort();
    let ans = (0..(1 << c.len()))
        .map(|bit| {
            let x = (0..c.len())
                .map(|i| if bit >> i & 1 == 0 { 0 } else { c[i] })
                .sum::<u64>();
            if x > t {
                return 0;
            }
            let j = s.upper_bound(&(t - x));
            if j == 0 {
                x
            } else {
                x + s[j - 1]
            }
        })
        .max();
    println!("{}", ans.unwrap());
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
