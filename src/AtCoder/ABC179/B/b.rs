fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let dd: Vec<(usize, usize)> = (0..n)
        .map(|_| {
            let d1: usize = rd.get();
            let d2: usize = rd.get();
            (d1, d2)
        })
        .collect();
    let found = dd.windows(3).any(|x| {
        let (d, e, f) = (x[0], x[1], x[2]);
        d.0 == d.1 && e.0 == e.1 && f.0 == f.1
    });
    println!("{}", if found { "Yes" } else { "No" });
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
