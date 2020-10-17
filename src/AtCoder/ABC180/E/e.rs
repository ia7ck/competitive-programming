macro_rules! chmin {
    ($a:expr, $b:expr) => {
        $a = std::cmp::min($a, $b)
    };
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let points: Vec<(i64, i64, i64)> = (0..n)
        .map(|_| {
            let x: i64 = rd.get();
            let y: i64 = rd.get();
            let z: i64 = rd.get();
            (x, y, z)
        })
        .collect();
    let dist = |u: usize, v: usize| {
        let (a, b, c) = points[u];
        let (p, q, r) = points[v];
        (p - a).abs() + (q - b).abs() + std::cmp::max(0, r - c)
    };
    let inf = std::i64::MAX / 2;
    let mut dp = vec![vec![inf; n]; 1 << n];
    dp[1][0] = 0;
    for bits in 0..(1 << n) {
        for u in 0..n {
            for v in 0..n {
                chmin!(dp[bits | (1 << v)][v], dp[bits][u] + dist(u, v));
            }
        }
    }
    println!("{}", dp[(1 << n) - 1][0]);
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
