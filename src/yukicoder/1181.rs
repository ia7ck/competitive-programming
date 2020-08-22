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

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let mo = 1000000000 + 7;
    let n: i64 = rd.get();
    let k: i64 = rd.get();
    let k = k % mo;
    let mut a = 1;
    let mut b = 1;
    for _ in 0..n {
        a = a * (k * (k + 3) / 2 % mo);
        a = a % mo;
        b = b * (k * (k + 1) / 2 % mo);
        b = b % mo;
    }
    println!("{}", (a - b + mo) % mo);
}
