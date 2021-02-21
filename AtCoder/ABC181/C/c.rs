fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let xy: Vec<(i32, i32)> = (0..n)
        .map(|_| {
            let x: i32 = rd.get();
            let y: i32 = rd.get();
            (x, y)
        })
        .collect();
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if i == j || j == k || k == i {
                    continue;
                }
                let (xi, yi) = xy[i];
                let (xj, yj) = xy[j];
                let (xk, yk) = xy[k];
                // xj - xi : yj - yi = xk - xi : yk - yi
                if (xj - xi) * (yk - yi) == (yj - yi) * (xk - xi) {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
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
