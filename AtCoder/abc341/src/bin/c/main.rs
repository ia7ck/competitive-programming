use proconio::{input, marker::Chars};

fn safe((si, sj): (usize, usize), s: &Vec<Vec<char>>, t: &Vec<char>) -> bool {
    if s[si][sj] == '#' {
        return false;
    }
    
    let (mut i, mut j) = (si, sj);
    for &ch in t {
        assert!(1 <= i && i + 1 < s.len());
        assert!(1 <= j && j + 1 < s[i].len());
        match ch {
            'L' => j -= 1,
            'R' => j += 1,
            'U' => i -= 1,
            'D' => i += 1,
            _ => unreachable!(),
        }
        if s[i][j] == '#' {
            return false;
        }
    }
    true
}

fn main() {
    input! {
        h: usize,
        w: usize,
        _n: usize,
        t: Chars,
        s: [Chars; h],
    };

    let mut ans = 0;
    for i in 1..(h - 1) {
        for j in 1..(w - 1) {
            if safe((i, j), &s, &t) {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
