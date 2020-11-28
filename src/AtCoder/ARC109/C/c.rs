fn pow(a: usize, x: usize, mo: usize) -> usize {
    if x == 0 {
        1
    } else if x == 1 {
        a
    } else if x % 2 == 0 {
        pow(a * a % mo, x / 2, mo)
    } else {
        a * pow(a, x - 1, mo) % mo
    }
}

fn janken(a: char, b: char) -> char {
    if a == 'R' {
        if b == 'S' {
            a
        } else {
            b
        }
    } else if a == 'P' {
        if b == 'R' {
            a
        } else {
            b
        }
    } else {
        if b == 'P' {
            a
        } else {
            b
        }
    }
}

use std::collections::HashMap;
fn solve(
    i: usize,
    k: usize,
    n: usize,
    s: &[char],
    memo: &mut HashMap<(usize, usize), char>,
) -> char {
    if k == 1 {
        let j = (i + 1) % n;
        return janken(s[i], s[j]);
    }
    if let Some(&res) = memo.get(&(i, k)) {
        return res;
    }
    let l = solve(i, k - 1, n, s, memo);
    let r = solve((i + pow(2, k - 1, n)) % n, k - 1, n, s, memo);
    let res = janken(l, r);
    memo.insert((i, k), res);
    res
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let k: usize = rd.get();
    let s: String = rd.get();
    let s: Vec<char> = s.chars().collect();

    let ans = solve(0, k, n, &s, &mut HashMap::new());
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
