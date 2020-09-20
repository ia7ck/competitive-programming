fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: u64 = rd.get();
    let x: usize = rd.get();
    let m: usize = rd.get();

    let b = 34;
    let mut nxt = vec![vec![0; m]; b];
    // dp[i][j]: x = j から始めたときの a[1] + a[2] + ... + a[2^i]
    let mut dp = vec![vec![0; m]; b];
    for y in 0..m {
        nxt[0][y] = (y * y) % m;
        dp[0][y] = y;
    }
    for i in 1..b {
        for j in 0..m {
            nxt[i][j] = nxt[i - 1][nxt[i - 1][j]];
            dp[i][j] = dp[i - 1][j] + dp[i - 1][nxt[i - 1][j]];
        }
    }
    let mut y = x;
    let mut ans = 0;
    for i in 0..b {
        if (n >> i & 1) == 1 {
            ans += dp[i][y];
            y = nxt[i][y];
        }
    }
    println!("{}", ans);
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
