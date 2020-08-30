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

fn gcd(a: usize, b: usize) -> usize {
    if a % b == 0 {
        b
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let a: Vec<usize> = (0..n).map(|_| rd.get()).collect();

    let mut pairwise = true;
    let mut seen = std::collections::HashSet::new();
    for &x in &a {
        if !pairwise {
            break;
        }
        let mut s = std::collections::HashSet::new();
        let mut y = x;
        let mut i = 2;
        while y > 1 {
            if i * i > y {
                break;
            }
            while y % i == 0 {
                s.insert(i);
                y /= i;
            }
            i += 1;
        }
        if y > 1 {
            s.insert(y);
        }
        for s in s {
            if !seen.insert(s) {
                pairwise = false;
                break;
            }
        }
    }

    if pairwise {
        println!("pairwise coprime");
        return;
    }

    let mut g = a[0];
    for i in 1..n {
        g = gcd(g, a[i]);
    }
    if g == 1 {
        println!("setwise coprime");
        return;
    }

    println!("not coprime");
}
