use std::collections::HashMap;

// s[l..r] に対する答え
fn solve(
    l: usize,
    r: usize,
    s: &Vec<char>,
    t: &Vec<char>,
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if l >= r {
        return 0;
    }
    if r - l <= 2 {
        return 0;
    }
    if let Some(&ans) = memo.get(&(l, r)) {
        return ans;
    }
    let mut ans = 0;
    ans = ans.max(solve(l + 1, r, s, t, memo));
    ans = ans.max(solve(l, r - 1, s, t, memo));
    for k in (l + 1)..(r - 1) {
        // [l, k), [k, r)
        ans = ans.max(solve(l, k, s, t, memo) + solve(k, r, s, t, memo));
        if (r - l) % 3 == 0 && [s[l], s[k], s[r - 1]] == [t[0], t[1], t[2]] {
            // aabcbabcc
            // a   b   c
            let lv = solve(l + 1, k, s, t, memo);
            let rv = solve(k + 1, r - 1, s, t, memo);
            if (k - (l + 1)) % 3 == 0
                && lv == (k - (l + 1)) / 3
                && (r - 1 - (k + 1)) % 3 == 0
                && rv == (r - 1 - (k + 1)) / 3
            {
                ans = ans.max((r - l) / 3);
            }
        }
    }
    memo.insert((l, r), ans);
    ans
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let s: Vec<char> = rd.get::<String>().chars().collect();
    let t: Vec<char> = rd.get::<String>().chars().collect();

    let mut memo = HashMap::new();
    let ans = solve(0, n, &s, &t, &mut memo);
    println!("{}", ans);
}

pub struct ProconReader<R> {
    r: R,
    line: String,
    i: usize,
}

impl<R: std::io::BufRead> ProconReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            r: reader,
            line: String::new(),
            i: 0,
        }
    }
    pub fn get<T>(&mut self) -> T
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.skip_blanks();
        assert!(self.i < self.line.len());
        assert_ne!(&self.line[self.i..=self.i], " ");
        let line = &self.line[self.i..];
        let end = line.find(' ').unwrap_or_else(|| line.len());
        let s = &line[..end];
        self.i += end;
        s.parse()
            .unwrap_or_else(|_| panic!("parse error `{}`", self.line))
    }
    fn skip_blanks(&mut self) {
        loop {
            let start = self.line[self.i..].find(|ch| ch != ' ');
            match start {
                Some(j) => {
                    self.i += j;
                    break;
                }
                None => {
                    self.line.clear();
                    self.i = 0;
                    let num_bytes = self.r.read_line(&mut self.line).unwrap();
                    assert!(num_bytes > 0, "reached EOF :(");
                    self.line = self.line.trim_end_matches(&['\r', '\n'][..]).to_string();
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
}
