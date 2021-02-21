fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let a: usize = rd.get();
    let b: usize = rd.get();
    let c: usize = rd.get();

    let n = 100;
    let mut dp = vec![vec![vec![None; n + 1]; n + 1]; n + 1];
    dp[a][b][c] = Some(1.0);
    for i in a..n {
        for j in b..n {
            for k in c..n {
                if dp[i][j][k].is_none() {
                    continue;
                }
                let x = dp[i][j][k].unwrap();
                // println!("{} {} {} {}", i, j, k, x);
                let m = (i + j + k) as f64;
                let y = dp[i + 1][j][k].get_or_insert(0.0);
                *y += x * (i as f64) / m;
                // println!("{}", *y);
                let y = dp[i][j + 1][k].get_or_insert(0.0);
                *y += x * (j as f64) / m;
                // println!("{}", *y);
                let y = dp[i][j][k + 1].get_or_insert(0.0);
                *y += x * (k as f64) / m;
                // println!("{}", *y);
            }
        }
    }
    let mut ans = 0.0;
    for i in a..n {
        for j in b..n {
            ans += dp[i][j][n].unwrap_or(0.0) * ((i - a + j - b + n - c) as f64);
        }
    }
    for j in b..n {
        for k in c..n {
            ans += dp[n][j][k].unwrap_or(0.0) * ((n - a + j - b + k - c) as f64);
        }
    }
    for i in a..n {
        for k in c..n {
            ans += dp[i][n][k].unwrap_or(0.0) * ((i - a + n - b + k - c) as f64);
        }
    }
    println!("{:.10}", ans);
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
