use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: Chars,
        u: String,
    };

    // 26^4 = 456976
    let mut q_pos = Vec::new();
    for i in 0..t.len() {
        if t[i] == '?' {
            q_pos.push(i);
        }
    }
    for c1 in 'a'..='z' {
        for c2 in 'a'..='z' {
            for c3 in 'a'..='z' {
                for c4 in 'a'..='z' {
                    let mut s = t.clone();
                    for (&i, &c) in q_pos.iter().zip(&[c1, c2, c3, c4]) {
                        s[i] = c;
                    }
                    let s = s.iter().collect::<String>();
                    // u is substring of s
                    if s.contains(&u) {
                        println!("Yes");
                        return;
                    }
                }
            }
        }
    }

    println!("No");
}
