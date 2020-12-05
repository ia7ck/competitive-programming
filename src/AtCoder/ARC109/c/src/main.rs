use std::fmt::{Display, Formatter};

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let k: usize = rd.get();
    let s: String = rd.get();
    let s: Vec<char> = s.chars().collect();

    #[derive(Clone, Debug)]
    enum Hand {
        R,
        P,
        S,
    }
    impl Display for Hand {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            use Hand::{P, R, S};
            match self {
                R => write!(f, "R"),
                P => write!(f, "P"),
                S => write!(f, "S"),
            }
        }
    }
    let judge = |l: Hand, r: Hand| {
        use Hand::{P, R, S};
        match (l, r) {
            (R, S) | (S, R) | (R, R) => R,
            (P, R) | (R, P) | (P, P) => P,
            (S, P) | (P, S) | (S, S) => S,
        }
    };
    let s = s
        .iter()
        .map(|ch| match ch {
            'R' => Hand::R,
            'P' => Hand::P,
            'S' => Hand::S,
            _ => unreachable!(),
        })
        .collect::<Vec<Hand>>();

    let ans = (0..k).fold(s, |t, _| {
        let u = t.iter().chain(t.iter()).collect::<Vec<_>>();
        (0..u.len())
            .step_by(2)
            .map(|i| {
                let l = u[i].clone();
                let r = u[i + 1].clone();
                judge(l, r)
            })
            .collect()
    });
    println!("{}", ans[0]);
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
