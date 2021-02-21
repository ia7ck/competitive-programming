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

macro_rules! chmax {
    ($a:expr, $b:expr) => {
        $a = std::cmp::max($a, $b)
    };
}

macro_rules! chmin {
    ($a:expr, $b:expr) => {
        $a = std::cmp::min($a, $b)
    };
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let m: i64 = rd.get();
    let a: Vec<i64> = (0..n).map(|_| rd.get()).collect();

    let mut dp = vec![0, 0];
    for i in (0..n).rev() {
        let x = a[i];
        let mut nxt = vec![0, 0];
        nxt[0] = dp[1] + (x - m);
        nxt[1] = dp[0] - (x - m);
        if x >= 2 {
            chmax!(nxt[0], dp[0] + (x - 1) - (1 - m));
            chmin!(nxt[1], dp[1] - (x - 1) + (1 - m));
        }
        dp = nxt;
    }
    println!("{}", if dp[0] > 0 { "First" } else { "Second" });
}
