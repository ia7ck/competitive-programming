fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let k: u64 = rd.get();
    let t: Vec<Vec<u64>> = (0..n).map(|_| (0..n).map(|_| rd.get()).collect()).collect();

    let mut perm: Vec<usize> = (0..n).collect();
    let mut ans = 0;
    loop {
        if perm[0] == 0 {
            let mut s = 0;
            for i in 0..n {
                s += t[perm[i]][perm[(i + 1) % n]];
            }
            if s == k {
                ans += 1;
            }
        }
        if !perm.next_permutation() {
            break;
        }
    }
    println!("{}", ans);
}

pub trait NextPermutation {
    fn next_permutation(&mut self) -> bool;
}

impl<T: Ord> NextPermutation for [T] {
    /// 数列を辞書順でひとつ進めます。進めなかったら false を返します。
    ///
    /// # Examples
    /// ```
    /// use next_permutation::NextPermutation;
    /// let mut a = vec![1, 2, 3];
    /// a.next_permutation();
    /// assert_eq!(a, vec![1, 3, 2]);
    /// a.next_permutation();
    /// assert_eq!(a, vec![2, 1, 3]);
    /// let mut a = vec![3, 2, 1];
    /// assert!(!a.next_permutation());
    /// ```
    fn next_permutation(&mut self) -> bool {
        if self.len() <= 0 {
            return false;
        }
        let mut i = self.len() - 1;
        while i > 0 && self[i - 1] >= self[i] {
            i -= 1;
        }
        if i == 0 {
            return false;
        }
        let mut j = self.len() - 1;
        while self[i - 1] >= self[j] {
            j -= 1;
        }
        self.swap(i - 1, j);
        self[i..].reverse();
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_test() {
        let mut a: Vec<i32> = vec![];
        assert!(!a.next_permutation());
    }

    #[test]
    fn one_test() {
        let mut a = vec![1];
        assert!(!a.next_permutation());
    }

    #[test]
    fn uniq_test() {
        let mut a = vec![1, 2, 3];
        let want = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];
        for i in 0..want.len() {
            assert_eq!(a, want[i]);
            if i < want.len() - 1 {
                assert_eq!(a.next_permutation(), true);
            } else {
                assert_eq!(a.next_permutation(), false);
            }
        }
    }
    #[test]
    fn general_test() {
        let mut a = vec![1, 2, 2, 3];
        let want = vec![
            vec![1, 2, 2, 3],
            vec![1, 2, 3, 2],
            vec![1, 3, 2, 2],
            vec![2, 1, 2, 3],
            vec![2, 1, 3, 2],
            vec![2, 2, 1, 3],
            vec![2, 2, 3, 1],
            vec![2, 3, 1, 2],
            vec![2, 3, 2, 1],
            vec![3, 1, 2, 2],
            vec![3, 2, 1, 2],
            vec![3, 2, 2, 1],
        ];
        for i in 0..want.len() {
            assert_eq!(a, want[i]);
            if i < want.len() - 1 {
                assert_eq!(a.next_permutation(), true);
            } else {
                assert_eq!(a.next_permutation(), false);
            }
        }
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
