fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let w: i64 = rd.get();
    let stp: Vec<(usize, usize, i64)> = (0..n)
        .map(|_| {
            let s: usize = rd.get();
            let t: usize = rd.get();
            let p: i64 = rd.get();
            (s, t, p)
        })
        .collect();

    let mut cul = vec![0; 2 * 100000 + 1];
    for (s, t, p) in stp {
        cul[s] += p;
        cul[t] -= p;
    }
    for i in 1..cul.len() {
        cul[i] += cul[i - 1];
    }
    println!(
        "{}",
        if cul.iter().all(|&x| x <= w) {
            "Yes"
        } else {
            "No"
        }
    );
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
