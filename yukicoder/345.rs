use std::io::Read;

fn read<T: std::str::FromStr>() -> T {
    let token: String = std::io::stdin()
        .bytes()
        .map(|c| c.ok().unwrap() as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().unwrap()
}

fn cww(s: &[char]) -> bool {
    let mut c = 0;
    let mut w = 0;
    for &x in s {
        if x == 'c' {
            c += 1;
        }
        if x == 'w' && c >= 1 {
            w += 1;
        }
        if c >= 1 && w >= 2 {
            return true;
        }
    }
    return false;
}

fn main() {
    let s: String = read();
    let s = s.chars().collect::<Vec<_>>();
    let mut ans = std::usize::MAX;
    for i in 0..s.len() {
        for j in i..s.len() {
            if cww(&s[i..=j]) {
                ans = std::cmp::min(ans, j - i + 1);
            }
        }
    }
    println!(
        "{}",
        if ans == std::usize::MAX {
            -1
        } else {
            ans as i32
        }
    );
}
