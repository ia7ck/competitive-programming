fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let p: Vec<usize> = (0..n).map(|_| rd.get()).collect();

    let mut p = p.iter().map(|&x| x - 1).collect::<Vec<_>>();
    let d_sum = p
        .iter()
        .zip(0..n)
        .map(|(&x, i)| if x >= i { x - i } else { i - x })
        .sum::<usize>();
    if d_sum != (n - 1) * 2 {
        println!("{}", -1);
        return;
    }
    let mut pos = vec![0; n];
    for i in 0..n {
        pos[p[i]] = i;
    }
    let mut ans = vec![];
    for x in 0..n {
        assert!(x <= pos[x]);
        while x < pos[x] {
            let i = pos[x] - 1;
            let j = pos[x];
            pos[p[i]] = j;
            pos[p[j]] = i;
            {
                let tmp = p[i];
                p[i] = p[j];
                p[j] = tmp;
            }
            // println!("{:?}", &p);
            ans.push(i);
        }
    }
    // assert_eq!(ans.len(), n - 1); // ??
    if ans.len() != n - 1 {
        println!("{}", -1);
        return;
    }
    use std::collections::HashSet;
    let set = ans.iter().collect::<HashSet<_>>();
    assert_eq!(set.len(), n - 1);
    println!(
        "{}",
        ans.iter()
            .map(|&x| (x + 1).to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
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
