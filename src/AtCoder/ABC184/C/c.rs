fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let r1: i64 = rd.get();
    let c1: i64 = rd.get();
    let r2: i64 = rd.get();
    let c2: i64 = rd.get();

    if r1 == r2 && c1 == c2 {
        println!("0");
        return;
    }
    if r1 + c1 == r2 + c2 || r1 - c1 == r2 - c2 || (r1 - r2).abs() + (c1 - c2).abs() <= 3 {
        println!("1");
        return;
    }

    let mut y = r2 - r1;
    let mut x = c2 - c1;
    if y < 0 {
        y *= -1;
    }
    if x < 0 {
        x *= -1;
    }
    if (x == 5 && y == 0) || (x == 0 && y == 5) {
        println!("2");
        return;
    }
    if (x + y) % 2 == 1 && (y >= x + 5 || y <= x - 5) {
        println!("3");
    } else {
        println!("2");
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
