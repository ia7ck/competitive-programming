fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let k: usize = rd.get();
    let td: Vec<(usize, u64)> = (0..n)
        .map(|_| {
            let t: usize = rd.get();
            let d: u64 = rd.get();
            (t, d)
        })
        .collect();

    let mut td = td;
    td.sort_by(|(_, d1), (_, d2)| d2.cmp(d1));
    use std::collections::HashSet;
    let mut freq = vec![0; n + 1];
    let mut kind = HashSet::new();
    for &(t, _) in &td[..k] {
        freq[t] += 1;
        kind.insert(t);
    }
    let mut kind = kind.len() as u64;
    let mut base = td[..k].iter().map(|(_, d)| d).sum::<u64>();
    let mut ans = base + kind * kind;
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let mut min_heap = td[..k]
        .iter()
        .map(|&(t, d)| Reverse((d, t)))
        .collect::<BinaryHeap<_>>();
    for i in k..n {
        let (t, d) = td[i];
        // 同じ種類のすしは取らない
        if freq[t] >= 1 {
            continue;
        }
        // 捨てるすしを小さいほうから考慮する
        while let Some(Reverse((dd, tt))) = min_heap.pop() {
            // 種類数が減らないように捨てる
            if freq[tt] >= 2 {
                freq[tt] -= 1;
                freq[t] += 1; // freq[t] = 1
                kind += 1;
                base = base - dd + d;
                ans = ans.max(base + kind * kind);
                break;
            }
        }
    }
    println!("{}", ans);
}

pub struct ProconReader<R> {
    r: R,
    l: String,
    i: usize,
}

impl<R: std::io::BufRead> ProconReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            r: reader,
            l: String::new(),
            i: 0,
        }
    }
    pub fn get<T>(&mut self) -> T
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.skip_blanks();
        assert!(self.i < self.l.len()); // remain some character
        assert_ne!(&self.l[self.i..=self.i], " ");
        let rest = &self.l[self.i..];
        let len = rest.find(' ').unwrap_or_else(|| rest.len());
        // parse self.l[self.i..(self.i + len)]
        let val = rest[..len]
            .parse()
            .unwrap_or_else(|e| panic!("{:?}, attempt to read `{}`", e, rest));
        self.i += len;
        val
    }
    fn skip_blanks(&mut self) {
        loop {
            match self.l[self.i..].find(|ch| ch != ' ') {
                Some(j) => {
                    self.i += j;
                    break;
                }
                None => {
                    let mut buf = String::new();
                    let num_bytes = self
                        .r
                        .read_line(&mut buf)
                        .unwrap_or_else(|_| panic!("invalid UTF-8"));
                    assert!(num_bytes > 0, "reached EOF :(");
                    self.l = buf
                        .trim_end_matches('\n')
                        .trim_end_matches('\r')
                        .to_string();
                    self.i = 0;
                }
            }
        }
    }
    pub fn get_vec<T>(&mut self, n: usize) -> Vec<T>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        (0..n).map(|_| self.get()).collect()
    }
    pub fn get_chars(&mut self) -> Vec<char> {
        self.get::<String>().chars().collect()
    }
}
