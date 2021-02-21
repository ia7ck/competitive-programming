fn naive(n: i64, k: i64) -> i64 {
    let mut res = 0;
    for a in 1..=n {
        for b in 1..=n {
            for c in 1..=n {
                for d in 1..=n {
                    if a + b - (c + d) == k {
                        res += 1;
                    }
                }
            }
        }
    }
    res
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: i64 = rd.get();
    let k: i64 = rd.get();

    // println!("{}", naive(n, k));

    let n = n as usize;
    let mut num = vec![0; n * 2 + 1];
    for i in 2..=(n * 2) {
        if i <= n + 1 {
            num[i] = num[i - 1] + 1;
        } else {
            num[i] = num[i - 1] - 1;
        }
    }
    let mut ans: u64 = 0;
    for ab in 2..=(n * 2) {
        let cd = ab as i64 - k;
        if 2 <= cd && cd <= (n * 2) as i64 {
            ans += num[ab] * num[cd as usize];
        }
    }
    // println!("{:?}", num);
    // println!("{}", naive(n as i64, k));
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
