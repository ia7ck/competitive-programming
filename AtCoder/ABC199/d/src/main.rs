fn dfs(
    i: usize,
    p: usize,
    seen: &mut Vec<bool>,
    nodes: &mut Vec<usize>,
    par: &mut Vec<usize>,
    g: &Vec<Vec<usize>>,
) {
    seen[i] = true;
    nodes.push(i);
    par.push(p);
    for &j in &g[i] {
        if !seen[j] {
            dfs(j, i, seen, nodes, par, g);
        }
    }
}

fn f(
    i: usize,
    colors: &mut Vec<i32>,
    nodes: &Vec<usize>,
    par: &Vec<usize>,
    g: &Vec<Vec<usize>>,
) -> u64 {
    if i == nodes.len() {
        let mut ok = true;
        for u in 0..g.len() {
            for &v in &g[u] {
                if colors[u] >= 0 && colors[v] >= 0 && colors[u] == colors[v] {
                    ok = false;
                    break;
                }
            }
        }
        return ok as u64;
    }
    let (v, p) = (nodes[i], par[i]);
    let mut res = 0;
    for c in 0..3 {
        if p == v || colors[p] != c {
            colors[v] = c;
            res += f(i + 1, colors, nodes, par, g);
            colors[v] = -1;
        }
    }
    res
}
fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let m: usize = rd.get();
    let mut g = vec![vec![]; n];
    for _ in 0..m {
        let a: usize = rd.get();
        let b: usize = rd.get();
        g[a - 1].push(b - 1);
        g[b - 1].push(a - 1);
    }
    let mut seen = vec![false; n];
    let mut ans = Vec::new();
    for s in 0..n {
        if seen[s] {
            continue;
        }
        let mut nodes = Vec::new();
        let mut par = Vec::new();
        dfs(s, s, &mut seen, &mut nodes, &mut par, &g);
        let mut colors = vec![-1; n];
        let tmp = f(0, &mut colors, &nodes, &par, &g);
        ans.push(tmp);
    }
    let ans = ans.into_iter().product::<u64>();
    println!("{}", ans);
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
