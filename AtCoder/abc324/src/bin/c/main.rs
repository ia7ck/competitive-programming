use proconio::{input, marker::Chars};

fn check(s: &Vec<char>, t: &Vec<char>) -> bool {
    if s.len() == t.len() {
        let mut diff = 0;
        for i in 0..s.len() {
            if s[i] != t[i] {
                diff += 1;
            }
        }
        diff <= 1
    } else if s.len() + 1 == t.len() {
        // abcc
        // abbcc
        let Some(j) = s
            .iter()
            .zip(t.iter())
            .position(|(c1, c2)| c1 != c2) else {
                return true;
            };
        s[j..] == t[(j + 1)..]
    } else if s.len() == t.len() + 1 {
        check(t, s)
    } else {
        false
    }
}

fn main() {
    input! {
        n: usize,
        t: Chars,
        ss: [Chars; n],
    };

    let mut ans = Vec::new();
    for i in 0..n {
        if check(&ss[i], &t) {
            ans.push(i + 1);
        }
    }
    println!("{}", ans.len());
    for i in 0..ans.len() {
        print!("{}", ans[i]);
        if i + 1 < ans.len() {
            print!(" ");
        }
    }
    println!();
}
