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

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let xy: Vec<(i64, i64)> = (0..n)
        .map(|_| {
            let x: i64 = rd.get();
            let y: i64 = rd.get();
            (x, y)
        })
        .collect();
    let pts = vec![
        xy.iter()
            .max_by(|(x1, y1), (x2, y2)| (-x1 - y1).cmp(&(-x2 - y2)))
            .unwrap(),
        xy.iter()
            .max_by(|(x1, y1), (x2, y2)| (x1 + y1).cmp(&(x2 + y2)))
            .unwrap(),
        xy.iter()
            .max_by(|(x1, y1), (x2, y2)| (x1 - y1).cmp(&(x2 - y2)))
            .unwrap(),
        xy.iter()
            .max_by(|(x1, y1), (x2, y2)| (-x1 + y1).cmp(&(-x2 + y2)))
            .unwrap(),
    ];
    let mut ans = 0;
    for (x, y) in &xy {
        for (xx, yy) in &pts {
            ans = std::cmp::max(ans, (x - xx).abs() + (y - yy).abs());
        }
    }
    println!("{}", ans);
}
