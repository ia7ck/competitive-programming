fn solve(n: usize, a: &[i64]) {
    if n == 2 {
        println!("{}", 0);
        return;
    }
    let f = |b: &[i64]| b.windows(2).fold(0i64, |acc, w| acc + (w[0] - w[1]).abs());
    let s = f(a);
    let t = a
        .windows(3)
        .map(|w| {
            let m = (w[0] + w[2]) / 2;
            ((w[0] - w[1]).abs() - (w[0] - m).abs()) + ((w[1] - w[2]).abs() - (m - w[2]).abs())
        })
        .max()
        .unwrap();
    let ans = s - t;
    let tmp = std::cmp::min(f(&a[1..]), f(&a[..(n - 1)]));
    println!("{}", std::cmp::min(ans, tmp));
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let t: usize = rd.get();
    for _ in 0..t {
        let n: usize = rd.get();
        let a: Vec<i64> = (0..n).map(|_| rd.get()).collect();
        solve(n, &a);
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
