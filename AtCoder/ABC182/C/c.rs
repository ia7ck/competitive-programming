fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: String = rd.get();
    let n: Vec<usize> = n.chars().map(|ch| ch as usize - '0' as usize).collect();

    let len = n.len();
    let mut one = 0;
    let mut two = 0;
    for d in &n {
        if d % 3 == 1 {
            one += 1;
        }
        if d % 3 == 2 {
            two += 1;
        }
    }
    let s = n.iter().sum::<usize>() % 3;
    match s {
        0 => {
            println!("0");
        }
        1 => {
            println!(
                "{}",
                if one >= 1 && len > 1 {
                    1
                } else if two >= 2 && len > 2 {
                    2
                } else {
                    -1
                }
            );
        }
        2 => {
            println!(
                "{}",
                if two >= 1 && len > 1 {
                    1
                } else if one >= 2 && len > 2 {
                    2
                } else {
                    -1
                }
            );
        }
        _ => unreachable!(),
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
