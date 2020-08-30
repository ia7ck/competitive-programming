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

    let m = 1_00 + 5;
    let mut min_facs = (0..m).map(|i| i).collect::<Vec<usize>>();
    for i in 2..m {
        // prime
        if min_facs[i] == i {
            let mut j = i * 2;
            while j < m {
                if min_facs[j] == j {
                    min_facs[j] = i;
                }
                j += i;
            }
        }
    }
    for i in 2..m {
        println!("{} {}", i, min_facs[i]);
    }

    let mut pairwise_coprime = true;
    let mut seen = std::collections::HashSet::new();
    for &x in &a {
        if !pairwise_coprime {
            break;
        }
        let mut facs = std::collections::HashSet::new();
        let mut y = x;
        while y > 1 {
            facs.insert(min_facs[y]);
            y = y / min_facs[y];
        }
        for f in facs {
            let exist = !seen.insert(f);
            if exist {
                pairwise_coprime = false;
                break;
            }
        }
    }
    if pairwise_coprime {
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
