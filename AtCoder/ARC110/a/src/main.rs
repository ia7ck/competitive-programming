fn gcd(a: i64, b: i64)->i64{
    if b==0{
        a
    }else{
        gcd(b,a%b)
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: i64 = rd.get();
    let mut lcm = 1;
    for i in 2..=n {
        lcm = lcm * i / gcd(lcm ,i);
    }
    let ans = lcm + 1;
    for i in 2..=n {
        assert_eq!(ans % i, 1);
    }
    assert!(ans <= 1_000_000_000_000_0);
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
