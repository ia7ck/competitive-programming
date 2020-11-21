fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let m: usize = rd.get();
    let edges: Vec<(usize, usize, usize)> = (0..m)
        .map(|_| {
            let u: usize = rd.get();
            let v: usize = rd.get();
            let c: usize = rd.get();
            (u - 1, v - 1, c - 1)
        })
        .collect();

    let mut g = vec![vec![]; n];
    for &(u, v, c) in &edges {
        g[u].push((v, c));
        g[v].push((u, c));
    }

    use std::collections::VecDeque;
    let mut ans = vec![None; n];
    let mut q = VecDeque::new();
    ans[0] = Some(0);
    q.push_back(0);
    while let Some(u) = q.pop_front() {
        let c_u = ans[u].unwrap();
        for &(v, c) in &g[u] {
            if ans[v].is_some() {
                continue;
            }
            if c == c_u {
                ans[v] = Some((c_u + 1) % n);
            } else {
                ans[v] = Some(c);
            }
            q.push_back(v);
        }
    }
    if ans.iter().any(|&a| a.is_none()) {
        println!("No"); // ?
    } else {
        ans.iter().map(|a| a.unwrap() + 1).for_each(|a| {
            println!("{}", a);
        });
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
