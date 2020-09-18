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

fn z_algorithm<T: PartialEq>(a: &[T]) -> Vec<usize> {
    let n = a.len();
    let mut z = vec![0; n];
    z[0] = n;
    let mut i = 1;
    let mut len = 0;
    while i < n {
        while i + len < n && a[len] == a[i + len] {
            len += 1;
        }
        z[i] = len;
        if len == 0 {
            i += 1;
            continue;
        }
        let mut j = 1;
        while j + z[j] < len {
            z[i + j] = z[j];
            j += 1;
        }
        i += j;
        len -= j;
    }
    z
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let s: String = rd.get();
    let s = s.chars().collect::<Vec<_>>();
    let mut ans = 0;
    for i in 0..n {
        let z = z_algorithm(&s[i..n]);
        for j in (i + 1)..n {
            ans = std::cmp::max(ans, std::cmp::min(j - i, z[j - i]));
        }
    }
    println!("{}", ans);
}
