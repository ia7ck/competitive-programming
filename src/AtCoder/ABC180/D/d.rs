fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let mut x: u64 = rd.get();
    let y: u64 = rd.get();
    let a: u64 = rd.get();
    let b: u64 = rd.get();
    let mut ans = (y - x - 1) / b;
    let mut t = 0;
    while x < y && x < (y + a - 1) / a {
        x = x * a;
        t += 1;
        ans = std::cmp::max(ans, t + (y - x - 1) / b);
        // println!("{}", t + (y - x - 1) / b);
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
