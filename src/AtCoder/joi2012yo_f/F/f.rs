const MO: u16 = 10000;

fn solve(
    a: &[usize],
    i: usize,
    less: bool,
    dir: Option<isize>,
    s: usize,
    m: usize,
    prev: Option<usize>,
    memo: &mut Vec<Vec<Vec<Vec<Vec<Option<u16>>>>>>,
) -> u16 {
    if i == a.len() {
        if prev.is_some() && s == 0 {
            1
        } else {
            0
        }
    } else {
        let less_u = less as usize;
        let dir_u = (dir.unwrap_or(0).signum() + 1) as usize;
        let prev_u = prev.unwrap_or(10);
        if let Some(ans) = memo[i][less_u][dir_u][s][prev_u] {
            return ans;
        }
        let u = if less { 9 } else { a[i] };
        let ans = (0..=u)
            .map(|x| {
                let l = less || x < u;
                if let Some(dir) = dir {
                    let d = x as isize - prev.unwrap() as isize;
                    if d * dir < 0 {
                        solve(a, i + 1, l, Some(d), (s * 10 + x) % m, m, Some(x), memo)
                    } else {
                        0
                    }
                } else if let Some(prev) = prev {
                    let d = x as isize - prev as isize;
                    if d != 0 {
                        solve(a, i + 1, l, Some(d), (s * 10 + x) % m, m, Some(x), memo)
                    } else {
                        0
                    }
                } else if x == 0 {
                    solve(a, i + 1, l, None, s, m, None, memo)
                } else {
                    solve(a, i + 1, l, None, (s * 10 + x) % m, m, Some(x), memo)
                }
            })
            .fold(0, |acc, res| (acc + res) % MO);
        memo[i][less_u][dir_u][s][prev_u] = Some(ans);
        ans
    }
}

fn is_zigzag(a: &[usize]) -> bool {
    if a.len() == 1 {
        return true;
    }
    if a.len() == 2 {
        return a[0] != a[1];
    }
    a.windows(3)
        .all(|w| (w[0] < w[1] && w[1] > w[2]) || (w[0] > w[1] && w[1] < w[2]))
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let a: String = rd.get();
    let a: Vec<usize> = a.chars().map(|c| c as usize - '0' as usize).collect();
    let b: String = rd.get();
    let b: Vec<usize> = b.chars().map(|c| c as usize - '0' as usize).collect();
    let m: usize = rd.get();

    let f = |c: &[usize]| {
        let mut memo = vec![vec![vec![vec![vec![None; 11]; m]; 3]; 2]; c.len()];
        solve(&c, 0, false, None, 0, m, None, &mut memo)
    };

    let ans = (f(&b) - f(&a) + MO) % MO;
    if is_zigzag(&a) {
        println!("{}", (ans + 1) % MO);
    } else {
        println!("{}", ans);
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
