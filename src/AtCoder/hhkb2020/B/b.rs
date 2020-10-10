fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let h: usize = rd.get();
    let w: usize = rd.get();
    let a: Vec<Vec<char>> = (0..h)
        .map(|_| {
            let r: String = rd.get();
            r.chars().collect::<Vec<char>>()
        })
        .collect();
    let dy = vec![-1, 0, 0, 1];
    let dx = vec![0, -1, 1, 0];
    let mut ans = 0;
    for y in 0..h {
        for x in 0..w {
            if a[y][x] == '#' {
                continue;
            }
            for k in 0..4 {
                let yy = y as i32 + dy[k];
                let xx = x as i32 + dx[k];
                if yy < 0 || yy >= h as i32 || xx < 0 || xx >= w as i32 {
                    continue;
                }
                let yy = yy as usize;
                let xx = xx as usize;
                if a[yy][xx] == '.' {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans / 2);
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
