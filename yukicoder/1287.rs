// ax + by = gcd(a, b) = g を満たす (x, y, g) を返す
fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        // ax + 0y = a
        // -> x = 1
        (1, 0, a)
    } else {
        // a = bq + r
        // -> b * (qx + y) + rx = g
        let (q, r) = (a / b, a % b);
        let (s, t, g) = ext_gcd(b, r);
        (t, s - q * t, g)
    }
}

fn mpow(a: i64, x: i64, m: i64) -> i64 {
    if x == 0 {
        1
    } else if x % 2 == 0 {
        mpow(a * a % m, x / 2, m)
    } else {
        a * mpow(a, x - 1, m) % m
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let p = 1_000_000_000 + 7;
    let t: usize = rd.get();
    for _ in 0..t {
        let x: i64 = rd.get();
        let k: i64 = rd.get();
        let (a, mut b, g) = ext_gcd(p - 1, k);
        assert_eq!(a * (p - 1) + b * k, g);
        b += p - 1;
        b %= p - 1;
        println!("{}", mpow(x, b, p));
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
