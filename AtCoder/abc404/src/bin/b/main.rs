use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
        t: [Chars; n],
    };

    let mut ans = usize::MAX;
    for r in 0..4 {
        let mut s = s.clone();
        for _ in 0..r {
            s = rotate(&s);
        }
        let d = diff(&s, &t);
        ans = ans.min(r + d);
    }

    println!("{}", ans);
}

fn diff(s: &Vec<Vec<char>>, t: &Vec<Vec<char>>) -> usize {
    let mut d = 0;
    for i in 0..s.len() {
        for j in 0..s[i].len() {
            if s[i][j] != t[i][j] {
                d += 1;
            }
        }
    }
    d
}

fn rotate(s: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut t = vec![vec!['?'; s.len()]; s[0].len()];
    for i in 0..s.len() {
        for j in 0..s[i].len() {
            t[j][s.len() - 1 - i] = s[i][j];
        }
    }
    t
}
