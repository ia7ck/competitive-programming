fn solve(n: usize, m: usize, a: &[usize], b: &[usize]) {
    let mut ans = 0;
    for &x in a {
        if b.iter().any(|&y| y == x) {
            ans += 1;
        }
    }
    println!("{}", ans);
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let t: usize = rd.get();
    for _ in 0..t {
        let n: usize = rd.get();
        let m: usize = rd.get();
        let a: Vec<usize> = (0..n).map(|_| rd.get()).collect();
        let b: Vec<usize> = (0..m).map(|_| rd.get()).collect();
        solve(n, m, &a, &b);
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
