fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let s: String = rd.get();
    let s: Vec<char> = s.chars().collect();

    let mut a = vec![];
    for c in s {
        if c == 'x' {
            let len = a.len();
            if len >= 2 && a[len - 2] == 'f' && a[len - 1] == 'o' {
                // ...pqfo
                a.pop();
                a.pop();
            // ...pq
            } else {
                a.push(c);
            }
        } else {
            a.push(c);
        }
    }
    println!("{}", a.len());
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
