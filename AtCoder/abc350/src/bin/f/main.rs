use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    };

    let mut pos = (0..s.len()).collect::<Vec<_>>();
    let mut stack = Vec::new();
    for (i, &c) in s.iter().enumerate() {
        if c == '(' {
            stack.push(i);
        } else if c == ')' {
            let j = stack.pop().unwrap();
            pos[i] = j;
            pos[j] = i;
        }
    }

    let mut ans = String::new();
    solve(0, s.len(), 0, &s, &pos, &mut ans);
    println!("{}", ans);
}

fn solve(l: usize, r: usize, depth: usize, s: &Vec<char>, pos: &Vec<usize>, ans: &mut String) {
    if depth % 2 == 0 {
        let mut i = l;
        while i < r {
            if s[i] == '(' {
                let j = pos[i];
                assert!(i < j);
                solve(i + 1, j, depth + 1, s, pos, ans);
                i = j + 1;
            } else if s[i] == ')' {
                unreachable!();
            } else {
                ans.push(s[i]);
                i += 1;
            }
        }
    } else {
        assert!(l >= 1);
        let mut i = r - 1;
        while i >= l {
            if s[i] == ')' {
                let j = pos[i];
                assert!(j < i);
                solve(j + 1, i, depth + 1, s, pos, ans);
                i = j - 1;
            } else if s[i] == '(' {
                unreachable!();
            } else {
                let c = if 'a' <= s[i] && s[i] <= 'z' {
                    char::from(s[i] as u8 - b'a' + b'A')
                } else if 'A' <= s[i] && s[i] <= 'Z' {
                    char::from(s[i] as u8 - b'A' + b'a')
                } else {
                    unreachable!()
                };
                ans.push(c);
                i -= 1;
            }
        }
    }
}
