fn solve(s: Vec<char>) {
    let mut ans = std::usize::MAX;
    let t = "atcoder$".chars().collect::<Vec<_>>();
    // s[..i] == t[..i]
    for i in 0..t.len() {
        let mut cnt = 0;
        let mut j = 0;
        let mut k = 0;
        while j < i && k < s.len() {
            if t[j] == s[k] {
                cnt += k - j;
                j += 1;
                k += 1;
            // atc
            // bbaaattccc
            } else {
                k += 1;
            }
        }
        // println!("{:?}", &t[..i]);
        // println!("{} {} {}", j, k, cnt);
        if j == i {
            while k < s.len() {
                if t[i] < s[k] {
                    // println!("{}", cnt + k - i);
                    ans = std::cmp::min(ans, cnt + k - i);
                    break;
                }
                k += 1;
            }
        }
    }
    if ans == std::usize::MAX {
        println!("{}", -1);
    } else {
        println!("{}", ans);
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let t: usize = rd.get();
    for _ in 0..t {
        let s: String = rd.get();
        solve(s.chars().collect::<Vec<_>>());
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
