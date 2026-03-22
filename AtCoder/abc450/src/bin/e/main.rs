use proconio::{input, marker::Bytes};

fn main() {
    input! {
        x: Bytes,
        y: Bytes,
        q: usize,
        queries: [(usize, usize, char); q],
    };

    let n = 90;
    let mut len = vec![0usize; n];
    let mut o = vec![[0usize; 26]; n];
    let mut o1 = vec![[0usize; 26]; x.len() + 1];
    let mut o2 = vec![[0usize; 26]; y.len() + 1];
    len[1] = x.len();
    len[2] = y.len();
    for i in 3..n {
        len[i] = len[i - 1].saturating_add(len[i - 2]);
    }
    for &c in &x {
        o[1][usize::from(c - b'a')] += 1;
    }
    for &c in &y {
        o[2][usize::from(c - b'a')] += 1;
    }
    for i in 3..n {
        for c in 0..26 {
            o[i][c] = o[i - 1][c].saturating_add(o[i - 2][c]);
        }
    }
    for i in 1..=x.len() {
        o1[i][usize::from(x[i - 1] - b'a')] += 1;
        for c in 0..26 {
            o1[i][c] += o1[i - 1][c];
        }
    }
    for i in 1..=y.len() {
        o2[i][usize::from(y[i - 1] - b'a')] += 1;
        for c in 0..26 {
            o2[i][c] += o2[i - 1][c];
        }
    }

    for (l, r, c) in queries {
        let c = c as usize - 'a' as usize;
        let fr = f(n - 1, r, c, &len, &o, &o1, &o2);
        let fl = f(n - 1, l - 1, c, &len, &o, &o1, &o2);
        println!("{}", fr - fl);
    }
}

fn f(
    i: usize,
    j: usize,
    c: usize,
    len: &Vec<usize>,
    o: &Vec<[usize; 26]>,
    o1: &Vec<[usize; 26]>,
    o2: &Vec<[usize; 26]>,
) -> usize {
    if i == 0 || j == 0 {
        return 0;
    }

    if i == 1 {
        assert!(j < o1.len());
        return o1[j][c];
    }
    if i == 2 {
        assert!(j < o2.len());
        return o2[j][c];
    }
    if j <= len[i - 1] {
        return f(i - 1, j, c, len, o, o1, o2);
    }
    let prefix = o[i - 1][c];
    let rest = f(i - 2, j - len[i - 1], c, len, o, o1, o2);
    prefix + rest
}
