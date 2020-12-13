fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let q: usize = rd.get();
    let a: Vec<u32> = rd.get_vec(n);
    let mut seg = SegmentTree::new(n, 0, |x, y| x ^ y);
    for i in 0..n {
        seg.update(i, a[i]);
    }
    for _ in 0..q {
        let t: usize = rd.get();
        match t {
            1 => {
                let i: usize = rd.get();
                let i = i - 1;
                let y: u32 = rd.get();
                let x = seg.get(i);
                seg.update(i, x ^ y);
            }
            2 => {
                let i: usize = rd.get();
                let j: usize = rd.get();
                let ans = seg.fold((i-1)..j);
                println!("{}", ans);
            }
            _ => unreachable!(),
        }
    }
}

struct SegmentTree<T, F> {
    n: usize,
    dat: Vec<T>,
    e: T,
    multiply: F,
}

#[allow(dead_code)]
impl<T, F> SegmentTree<T, F>
where
    T: Clone + Copy + std::fmt::Debug,
    F: Fn(T, T) -> T,
{
    fn new(n: usize, e: T, multiply: F) -> Self {
        let n = n.next_power_of_two();
        Self {
            n,
            dat: vec![e; n * 2 - 1],
            e,
            multiply,
        }
    }
    fn get(&self, i: usize) -> T {
        self.dat[i + self.n - 1]
    }
    fn update(&mut self, i: usize, x: T) {
        let mut k = i + self.n - 1;
        self.dat[k] = x;
        while k > 0 {
            k = (k - 1) / 2;
            self.dat[k] = (self.multiply)(self.dat[k * 2 + 1], self.dat[k * 2 + 2]);
        }
    }
    fn fold(&self, range: std::ops::Range<usize>) -> T {
        self._fold(&range, 0, 0..self.n)
    }
    fn _fold(
        &self,
        range: &std::ops::Range<usize>,
        i: usize,
        i_range: std::ops::Range<usize>,
    ) -> T {
        if range.end <= i_range.start || i_range.end <= range.start {
            return self.e;
        }
        if range.start <= i_range.start && i_range.end <= range.end {
            return self.dat[i];
        }
        let m = (i_range.start + i_range.end) / 2;
        (self.multiply)(
            self._fold(range, i * 2 + 1, i_range.start..m),
            self._fold(range, i * 2 + 2, m..i_range.end),
        )
    }
}

pub struct ProconReader<R> {
    r: R,
    line: String,
    i: usize,
}

impl<R: std::io::BufRead> ProconReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            r: reader,
            line: String::new(),
            i: 0,
        }
    }
    pub fn get<T>(&mut self) -> T
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.skip_blanks();
        assert!(self.i < self.line.len());
        assert_ne!(&self.line[self.i..=self.i], " ");
        let line = &self.line[self.i..];
        let end = line.find(' ').unwrap_or_else(|| line.len());
        let s = &line[..end];
        self.i += end;
        s.parse()
            .unwrap_or_else(|_| panic!("parse error `{}`", self.line))
    }
    fn skip_blanks(&mut self) {
        loop {
            let start = self.line[self.i..].find(|ch| ch != ' ');
            match start {
                Some(j) => {
                    self.i += j;
                    break;
                }
                None => {
                    self.line.clear();
                    self.i = 0;
                    let num_bytes = self.r.read_line(&mut self.line).unwrap();
                    assert!(num_bytes > 0, "reached EOF :(");
                    self.line = self.line.trim_end_matches(&['\r', '\n'][..]).to_string();
                }
            }
        }
    }

    pub fn get_vec<T>(&mut self, n: usize) -> Vec<T>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        (0..n).map(|_| self.get()).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::ProconReader;
    use std::io::Cursor;

    fn get<T>(input: &str) -> T
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        ProconReader::new(Cursor::new(input)).get()
    }

    #[test]
    fn test_single() {
        assert_eq!(get::<usize>("123"), 123);
        assert_eq!(get::<i32>("-123"), -123);
        assert_eq!(get::<char>("a"), 'a');
        assert_eq!(get::<String>("abc"), "abc");
    }

    #[test]
    fn test_space_separated() {
        let input = "123 -123 a abc";
        let mut rd = ProconReader::new(Cursor::new(input));
        assert_eq!(rd.get::<usize>(), 123);
        assert_eq!(rd.get::<i32>(), -123);
        assert_eq!(rd.get::<char>(), 'a');
        assert_eq!(rd.get::<String>(), "abc");
    }

    #[test]
    fn test_line_separated() {
        let input = "123\n-123\n\n\na\nabc";
        let mut rd = ProconReader::new(Cursor::new(input));
        assert_eq!(rd.get::<usize>(), 123);
        assert_eq!(rd.get::<i32>(), -123);
        assert_eq!(rd.get::<char>(), 'a');
        assert_eq!(rd.get::<String>(), "abc");
    }

    fn get_vec<T>(input: &str, n: usize) -> Vec<T>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        ProconReader::new(Cursor::new(input)).get_vec(n)
    }

    #[test]
    fn test_get_vec() {
        assert_eq!(get_vec::<i32>("1 23 -456", 3), vec![1, 23, -456]);
        assert_eq!(get_vec::<String>("abc\nde\nf", 3), vec!["abc", "de", "f"]);
    }

    #[test]
    #[should_panic(expected = "reached EOF")]
    fn too_many_get() {
        let input = "123";
        let mut rd = ProconReader::new(Cursor::new(input));
        rd.get::<usize>(); // 123
        rd.get::<usize>();
    }

    #[test]
    #[should_panic]
    fn cannot_parse_string_as_char() {
        let input = "abc";
        let mut rd = ProconReader::new(Cursor::new(input));
        rd.get::<char>(); // mismatch type
    }
}
