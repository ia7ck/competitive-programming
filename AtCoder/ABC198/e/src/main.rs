fn dfs(
    i: usize,
    p: usize,
    g: &Vec<Vec<usize>>,
    colors: &[usize],
    cnt: &mut Vec<usize>,
    ans: &mut Vec<usize>,
) {
    let c = colors[i];
    if cnt[c] == 0 {
        ans.push(i);
    }
    cnt[c] += 1;
    for &j in &g[i] {
        if j == p {
            continue;
        }
        dfs(j, i, g, colors, cnt, ans);
    }
    cnt[c] -= 1;
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let colors: Vec<usize> = rd.get_vec(n);
    let mut g = vec![vec![]; n];
    for _ in 0..(n - 1) {
        let a: usize = rd.get();
        let b: usize = rd.get();
        g[a - 1].push(b - 1);
        g[b - 1].push(a - 1);
    }
    let mut cnt = vec![0; 1_000_00 + 1];
    let mut ans = Vec::new();
    dfs(0, std::usize::MAX, &g, &colors, &mut cnt, &mut ans);
    ans.sort();
    for ans in ans {
        println!("{}", ans + 1);
    }
}

pub struct ProconReader<R> {
    r: R,
    l: String,
    i: usize,
}

impl<R: std::io::BufRead> ProconReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            r: reader,
            l: String::new(),
            i: 0,
        }
    }
    pub fn get<T>(&mut self) -> T
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.skip_blanks();
        assert!(self.i < self.l.len()); // remain some character
        assert_ne!(&self.l[self.i..=self.i], " ");
        let rest = &self.l[self.i..];
        let len = rest.find(' ').unwrap_or_else(|| rest.len());
        // parse self.l[self.i..(self.i + len)]
        let val = rest[..len]
            .parse()
            .unwrap_or_else(|e| panic!("{:?}, attempt to read `{}`", e, rest));
        self.i += len;
        val
    }
    fn skip_blanks(&mut self) {
        loop {
            match self.l[self.i..].find(|ch| ch != ' ') {
                Some(j) => {
                    self.i += j;
                    break;
                }
                None => {
                    let mut buf = String::new();
                    let num_bytes = self
                        .r
                        .read_line(&mut buf)
                        .unwrap_or_else(|_| panic!("invalid UTF-8"));
                    assert!(num_bytes > 0, "reached EOF :(");
                    self.l = buf
                        .trim_end_matches('\n')
                        .trim_end_matches('\r')
                        .to_string();
                    self.i = 0;
                }
            }
        }
    }
    pub fn get_vec<T>(&mut self, n: usize) -> Vec<T>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        (0..n).map(|_| self.get()).collect()
    }
    pub fn get_chars(&mut self) -> Vec<char> {
        self.get::<String>().chars().collect()
    }
}
