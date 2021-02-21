fn tak(a: &Vec<(usize, usize)>) -> usize {
    let mut a = a.iter().cloned().collect::<Vec<_>>();
    a.sort_by_key(|p| p.1);
    let mut rr = 0;
    let mut ans = 0;
    for (l, r) in a {
        if rr < l {
            rr = r;
            ans += 1;
        }
    }
    ans
}

fn aok(a: &Vec<(usize, usize)>) -> usize {
    let mut a = a.iter().cloned().collect::<Vec<_>>();
    a.sort_by_key(|p| p.0);
    let mut rr = 0;
    let mut ans = 0;
    for (l, r) in a {
        if rr < l {
            ans += 1;
        }
        rr = std::cmp::max(rr, r);
    }
    ans
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: i32 = rd.get();
    let m: i32 = rd.get();
    if n == 1 && m == 0 {
        println!("1 2");
        return;
    }
    if m < 0 || m >= n - 1 {
        println!("-1");
        return;
    }
    let n = n as usize;
    let m = m as usize;

    let mut ans = vec![];
    for i in 0..(n - m - 1) {
        let l = i + 1;
        let r = 1_000_000_000 - l;
        ans.push((l, r));
    }
    for i in 0..(m + 1) {
        let l = i * 2 + 1 + 5_000_000;
        let r = l + 1;
        ans.push((l, r));
    }
    let ta = tak(&ans);
    let ao = aok(&ans);
    assert_eq!(ta - ao, m);
    println!(
        "{}",
        ans.iter()
            .map(|(l, r)| format!("{} {}", l, r))
            .collect::<Vec<_>>()
            .join("\n")
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
