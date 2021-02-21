fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let q: usize = rd.get();
    let c: Vec<usize> = (0..n).map(|_| rd.get()).collect();

    let c: Vec<usize> = c.iter().map(|v| v - 1).collect();
    let mut uf = UnionFind::new(n, &c);
    for _ in 0..q {
        let tp: usize = rd.get();
        if tp == 1 {
            let a: usize = rd.get();
            let b: usize = rd.get();
            uf.unite(a - 1, b - 1);
        } else {
            let x: usize = rd.get();
            let y: usize = rd.get();
            println!("{}", uf.query(x - 1, y - 1));
        }
    }
}

use std::collections::HashMap;

pub struct UnionFind {
    n: usize,
    par: Vec<usize>,
    size: Vec<usize>,
    freq: Vec<HashMap<usize, usize>>,
}

impl UnionFind {
    pub fn new(n: usize, classes: &Vec<usize>) -> UnionFind {
        let mut freq = vec![HashMap::new(); n];
        for (i, &cls) in classes.iter().enumerate() {
            freq[i].insert(cls, 1);
        }
        UnionFind {
            n,
            par: (0..n).map(|i| i).collect::<Vec<_>>(),
            size: vec![1; n],
            freq,
        }
    }
    pub fn find(&mut self, i: usize) -> usize {
        if self.par[i] == i {
            self.par[i]
        } else {
            self.par[i] = self.find(self.par[i]);
            self.par[i]
        }
    }
    pub fn unite(&mut self, i: usize, j: usize) {
        let i = self.find(i);
        let j = self.find(j);
        if i == j {
            return;
        }
        let (i, j) = if self.size[i] >= self.size[j] {
            (i, j)
        } else {
            (j, i)
        };
        self.par[j] = i;
        self.size[i] += self.size[j];
        let kv: Vec<(usize, usize)> = self.freq[j].drain().collect();
        self.freq[j].shrink_to_fit();
        for (cls, num) in kv {
            let f = self.freq[i].entry(cls).or_insert(0);
            *f += num;
        }
    }
    pub fn query(&mut self, x: usize, y: usize) -> usize {
        let p = self.find(x);
        *self.freq[p].get(&y).unwrap_or(&0)
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
