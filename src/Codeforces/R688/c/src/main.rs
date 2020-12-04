fn solve(n: usize, a: &Vec<Vec<usize>>) {
    let mut ymins = vec![n; 10];
    let mut ymaxs = vec![1; 10];
    let mut xmins = vec![n; 10];
    let mut xmaxs = vec![1; 10];
    for i in 0..n {
        for j in 0..n {
            let d = a[i][j];
            let y = i + 1;
            let x = j + 1;
            ymins[d] = std::cmp::min(ymins[d], y);
            ymaxs[d] = std::cmp::max(ymaxs[d], y);
            xmins[d] = std::cmp::min(xmins[d], x);
            xmaxs[d] = std::cmp::max(xmaxs[d], x);
        }
    }
    macro_rules! chmax {
        ($a:expr, $b:expr) => {
            $a = std::cmp::max($a, $b);
        };
    }
    let mut ans = vec![0; 10];
    for i in 0..n {
        for j in 0..n {
            let d = a[i][j];
            let y = i + 1;
            let x = j + 1;
            let ymin = ymins[d];
            let ymax = ymaxs[d];
            let xmin = xmins[d];
            let xmax = xmaxs[d];
            let mut s = 0;
            chmax!(s, (n - y) * (xmax - x));
            chmax!(s, (n - y) * (x - xmin));
            chmax!(s, (n - x) * (ymax - y));
            chmax!(s, (n - x) * (y - ymin));
            chmax!(s, (y - 1) * (xmax - x));
            chmax!(s, (y - 1) * (x - xmin));
            chmax!(s, (x - 1) * (ymax - y));
            chmax!(s, (x - 1) * (y - ymin));
            chmax!(ans[d], s);
        }
    }
    // println!("ymin={} ymax={} xmin={} xmax={}", ymins[2], ymaxs[2], xmins[2], xmaxs[2]);
    println!(
        "{}",
        ans.iter()
            .map(|ans| ans.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let t: usize = rd.get();
    for _ in 0..t {
        let n: usize = rd.get();
        let a: Vec<Vec<usize>> = (0..n)
            .map(|_| {
                let s: String = rd.get();
                s.chars().map(|ch| ch as usize - '0' as usize).collect()
            })
            .collect();
        solve(n, &a);
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
