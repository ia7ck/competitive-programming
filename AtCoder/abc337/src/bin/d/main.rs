use proconio::{input, marker::Chars};

fn g(s: &Vec<char>, k: usize) -> Option<usize> {
    for &ch in s {
        assert!(ch == 'o' || ch == '.');
    }
    if s.len() < k {
        return None;
    }
    let mut dot = 0;
    for i in 0..k {
        if s[i] == '.' {
            dot += 1;
        }
    }
    let mut ans = dot;
    for i in k..s.len() {
        if s[i - k] == '.' {
            dot -= 1;
        }
        if s[i] == '.' {
            dot += 1;
        }
        ans = ans.min(dot);
    }
    Some(ans)
}

fn f(s: &Vec<char>, k: usize) -> Option<usize> {
    for &ch in s {
        assert!(ch == 'o' || ch == 'x' || ch == '.');
    }
    let s = {
        let mut s = s.clone();
        s.push('x'); // sentinel
        s
    };
    let mut ans = None;
    // split by 'x'
    let mut t = Vec::new();
    for ch in s {
        if ch == 'x' {
            if let Some(cost) = g(&t, k) {
                ans = match ans {
                    None => Some(cost),
                    Some(ans) => Some(ans.min(cost)),
                };
            }
            t.clear();
        } else {
            t.push(ch);
        }
    }
    ans
}

fn solve(s: &Vec<Vec<char>>, k: usize) -> Option<usize> {
    let mut ans = None;
    for s in s {
        if let Some(cost) = f(&s, k) {
            ans = match ans {
                None => Some(cost),
                Some(ans) => Some(ans.min(cost)),
            };
        }
    }
    ans
}

fn transpose(s: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let h = s.len();
    let w = s[0].len();
    let mut t = vec![vec!['.'; h]; w];
    for i in 0..h {
        for j in 0..w {
            t[j][i] = s[i][j];
        }
    }
    t
}

fn main() {
    input! {
        h: usize,
        _w: usize,
        k: usize,
        s: [Chars; h],
    };

    let ans1 = solve(&s, k);
    let ans2 = solve(&transpose(&s), k);
    match (ans1, ans2) {
        (None, None) => println!("-1"),
        (Some(ans1), None) => println!("{}", ans1),
        (None, Some(ans2)) => println!("{}", ans2),
        (Some(ans1), Some(ans2)) => println!("{}", ans1.min(ans2)),
    }
}
