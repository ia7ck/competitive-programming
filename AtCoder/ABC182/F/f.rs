fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let x: i64 = rd.get();
    let a: Vec<i64> = (0..n).map(|_| rd.get()).collect();

    let a = a.into_iter().chain(vec![std::i64::MAX]).collect::<Vec<_>>();
    let mut dp0: i64 = 1;
    let mut dp1: i64 = 0;
    for i in 0..n {
        let mut nxt0 = 0;
        let mut nxt1 = 0;
        let q = x / a[i];
        let y0 = q * a[i]; // 端数をぴったり払ってきたとき
        let y1 = (q + 1) * a[i]; // 繰り上がりがあるとき
        if y0 % a[i + 1] == 0 {
            nxt0 += dp0;
        } else {
            nxt0 += dp0;
            nxt1 += dp0; // a[i] を使わない, a[i+1] 以降で払う
        }
        if y1 % a[i + 1] == 0 {
            nxt1 += dp1;
        } else {
            nxt0 += dp1;
            nxt1 += dp1;
        }
        dp0 = nxt0;
        dp1 = nxt1;
    }
    println!("{}", dp0);
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
