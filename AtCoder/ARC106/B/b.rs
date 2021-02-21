fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let m: usize = rd.get();
    let a: Vec<i64> = (0..n).map(|_| rd.get()).collect();
    let b: Vec<i64> = (0..n).map(|_| rd.get()).collect();

    let mut g = vec![vec![]; n];
    for _ in 0..m {
        let c: usize = rd.get();
        let d: usize = rd.get();
        g[c - 1].push(d - 1);
        g[d - 1].push(c - 1);
    }

    let ok = |vs: &Vec<usize>| {
        let mut sa = 0;
        let mut sb = 0;
        for &v in vs {
            sa += a[v];
            sb += b[v];
        }
        sa == sb
    };

    let mut seen = vec![false; n];
    for s in 0..n {
        if seen[s] {
            continue;
        }
        let mut vs = vec![];
        let mut q = std::collections::VecDeque::new();
        seen[s] = true;
        q.push_back(s);
        while let Some(x) = q.pop_front() {
            vs.push(x);
            for &y in &g[x] {
                if seen[y] {
                    continue;
                }
                seen[y] = true;
                q.push_back(y);
            }
        }
        if !ok(&vs) {
            println!("No");
            return;
        }
    }
    println!("Yes");
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
