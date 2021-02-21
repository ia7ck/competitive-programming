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

mod rolling_hash {
    const M: u64 = 1_000_000_000 + 9;
    const B: u64 = 999999937;
    pub struct RollingHash {
        h: Vec<u64>,
        p: Vec<u64>,
    }
    impl RollingHash {
        pub fn new(s: &[u64]) -> Self {
            let n = s.len();
            let mut h = vec![0; n + 1];
            let mut p = vec![0; n + 1];
            p[0] = 1;
            for i in 0..n {
                h[i + 1] = (h[i] * B + s[i]) % M;
                p[i + 1] = p[i] * B % M;
            }
            Self { h, p }
        }
        pub fn get(&self, range: std::ops::Range<usize>) -> u64 {
            let l = range.start;
            let r = range.end;
            (self.h[r] + M - self.h[l] * self.p[r - l] % M) % M
        }
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    use rolling_hash::RollingHash;

    let n: usize = rd.get();
    let s: String = rd.get();
    let t = s.chars().map(|c| c as u64).collect::<Vec<_>>();
    let rh = RollingHash::new(&t);
    let mut ans = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            let mut ok = 0;
            let mut ng = std::cmp::min(j - i, n - j) + 1;
            while ng - ok > 1 {
                let m = (ng + ok) / 2;
                if rh.get(i..(i + m)) == rh.get(j..(j + m)) {
                    ok = m;
                } else {
                    ng = m;
                }
            }
            ans = std::cmp::max(ans, ok);
        }
    }
    println!("{}", ans);
}
