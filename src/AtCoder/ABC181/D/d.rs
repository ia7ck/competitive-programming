fn naive(s: &mut Vec<char>) {
    s.sort();
    if s.iter().collect::<String>().parse::<i32>().unwrap() % 8 == 0 {
        println!("Yes");
        return;
    }
    while s.next_permutation() {
        if s.iter().collect::<String>().parse::<i32>().unwrap() % 8 == 0 {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let s: String = rd.get();
    let mut s: Vec<char> = s.chars().collect();
    if s.len() <= 3 {
        naive(&mut s);
        return;
    }
    let s = s
        .iter()
        .map(|&c| c as usize - '0' as usize)
        .collect::<Vec<_>>();
    let mut freq = vec![0; 10];
    for &d in &s {
        freq[d] += 1;
    }
    for k in 100..=999 {
        if k % 8 != 0 {
            continue;
        }
        let mut f = vec![0; 10];
        f[k % 10] += 1;
        f[(k / 10) % 10] += 1;
        f[(k / 100) % 10] += 1;
        let ok = (0..10).all(|d| f[d] <= freq[d]);
        if ok {
            println!("Yes");
            return;
        }
    }
    println!("No");
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
