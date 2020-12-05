fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let s: String = rd.get();
    let s: Vec<char> = s.chars().collect();

    let total: u64 = 1_000_000_000_0;
    if n == 1 {
        println!("{}", if s[0] == '0' { total } else { total * 2 });
        return;
    }
    let mut cnt: u64 = 0;
    let mut cur: usize = if s[0..2] == ['1', '0'] {
        cnt += 1;
        2
    } else if s[0] == '0' {
        cnt += 1;
        1
    } else {
        0
    };
    while cur + 2 < n {
        if s[cur..=(cur + 2)] == ['1', '1', '0'] {
            cnt += 1;
            cur += 3;
        } else {
            break;
        }
    }
    if cur + 1 < n && s[cur..=(cur + 1)] == ['1', '1'] {
        cur += 2;
        cnt += 1;
    } else if cur < n && s[cur] == '1' {
        cur += 1;
        cnt += 1;
    }
    let ans = if cur == n { total - cnt + 1 } else { 0 };
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
