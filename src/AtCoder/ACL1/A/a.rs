fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let mut xyi: Vec<(usize, usize, usize)> = (0..n)
        .map(|i| {
            let x: usize = rd.get();
            let y: usize = rd.get();
            (x, y, i)
        })
        .collect();
    xyi.sort();
    let mut uf = UnionFind::new(n);
    let mut ys: Vec<(usize, usize)> = vec![];
    for (_, y, i) in xyi {
        let mut min = (y, i);
        while let Some(&(yy, ii)) = ys.last() {
            if y < yy {
                break;
            }
            uf.unite(i, ii);
            min = std::cmp::min(min, (yy, ii));
            ys.pop();
        }
        ys.push(min);
    }
    println!(
        "{}",
        (0..n)
            .map(|i| uf.get_size(i).to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
}

pub struct UnionFind {
    par: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        UnionFind {
            par: (0..n).map(|i| i).collect::<Vec<_>>(),
            size: vec![1; n],
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
    }
    pub fn get_size(&mut self, i: usize) -> usize {
        let p = self.find(i);
        self.size[p]
    }
    pub fn same(&mut self, i: usize, j: usize) -> bool {
        self.find(i) == self.find(j)
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
