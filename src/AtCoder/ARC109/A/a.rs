fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let a: u32 = rd.get();
    let b: u32 = rd.get();
    let x: u32 = rd.get();
    let y: u32 = rd.get();

    let ans = if a == b {
        x
    } else if a > b {
        let d = a - 1 - b;
        std::cmp::min(x + (a - 1 - b) * y, x * (d * 2 + 1))
    } else {
        let d = b - a;
        std::cmp::min(x + (b - a) * y, x * (d * 2 + 1))
    };
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
