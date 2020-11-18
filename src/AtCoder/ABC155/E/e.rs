use std::collections::HashMap;

// c=0 -> ぴったり
// c=1 -> 繰り上がりあり
fn solve(
    a: &[usize],
    i: Option<usize>,
    c: usize,
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    match i {
        None => c,
        Some(i) => {
            if let Some(&res) = memo.get(&(i, c)) {
                return res;
            }
            let x = a[i] + c;
            let ni = i.checked_sub(1);
            let res = if x == 10 {
                solve(a, ni, 1, memo)
            } else {
                std::cmp::min(x + solve(a, ni, 0, memo), (10 - x) + solve(a, ni, 1, memo))
            };
            memo.insert((i, c), res);
            res
        }
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: String = rd.get();
    let a: Vec<usize> = n.chars().map(|c| c as usize - '0' as usize).collect();

    let mut memo = HashMap::new();
    println!("{}", solve(&a, Some(a.len() - 1), 0, &mut memo));
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
