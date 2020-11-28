fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: u64 = rd.get();
    let mut a = 1;
    // while a * (a + 1) / 2 <= n + 1 {
    while a * (a + 1) <= (n + 1) * 2 {
        a += 1;
    }
    a -= 1;
    let b = n + 1 - a * (a + 1) / 2;
    eprintln!("{} {}", a, b);
    if b <= a {
        println!("{}", n + 1 - a);
    } else {
        println!("{}", n + 1 - a - 1);
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
