fn naive(n: usize, x: usize, m: usize) -> usize {
    let mut ans = 0;
    let mut y = x;
    for _ in 0..n {
        ans += y;
        y = y * y % m;
    }
    ans
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let x: usize = rd.get();
    let m: usize = rd.get();

    // println!("{}", naive(n, x, m));
    //
    // let mut y = x;
    // let mut seen = std::collections::HashSet::new();
    // for i in 0..1000 {
    // println!("{} {}", i, y);
    // if !seen.insert(y) {
    // break;
    // }
    // y = y * y % m;
    // }

    let mut last = vec![0; m];
    let mut a = vec![];
    let mut y = x;
    for i in 0..n {
        if last[y] > 0 {
            let p = last[y];
            let q = i - p;
            let r = (n - p) / q;
            assert!(r >= 1);
            let s = (n - p) % q;
            // println!("{:?}", &a[0..p]);
            // println!("{:?}", &a[p..]);
            // println!("{:?}", &a[p..(p + s)]);
            let ans = a[0..p].iter().sum::<usize>()
                + a[p..].iter().sum::<usize>() * r
                + a[p..(p + s)].iter().sum::<usize>();
            println!("{}", ans);
            return;
        }
        a.push(y);
        last[y] = i;
        y = y * y % m;
    }
    println!("{}", a.iter().sum::<usize>());
}

/*

17 2 1001
=> 4838

*/

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
