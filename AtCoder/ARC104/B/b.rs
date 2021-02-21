fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let s: String = rd.get();
    let s: Vec<char> = s.chars().collect();

    let mut aa = vec![0; n + 1];
    let mut tt = vec![0; n + 1];
    let mut cc = vec![0; n + 1];
    let mut gg = vec![0; n + 1];
    for i in 0..n {
        match s[i] {
            'A' => aa[i + 1] += 1,
            'T' => tt[i + 1] += 1,
            'C' => cc[i + 1] += 1,
            'G' => gg[i + 1] += 1,
            _ => unreachable!(),
        }
    }
    for i in 0..n {
        aa[i + 1] += aa[i];
        tt[i + 1] += tt[i];
        cc[i + 1] += cc[i];
        gg[i + 1] += gg[i];
    }
    let mut ans = 0;
    for i in 0..n {
        for j in i..n {
            // [i, j]
            let a = aa[j + 1] - aa[i];
            let t = tt[j + 1] - tt[i];
            let c = cc[j + 1] - cc[i];
            let g = gg[j + 1] - gg[i];
            // println!("{} {} {} {} {} {}", i, j, a, t, c, g);
            if a == t && c == g {
                ans += 1;
            }
        }
    }
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
