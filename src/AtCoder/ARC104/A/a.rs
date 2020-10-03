fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let a: i32 = rd.get();
    let b: i32 = rd.get();

    for x in -1000..=1000 {
        for y in -1000..=1000 {
            if x + y == a && x - y == b {
                println!("{} {}", x, y);
                return;
            }
        }
    }
    unreachable!();
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
