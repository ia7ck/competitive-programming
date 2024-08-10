use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: [Chars; n],
    };

    let l = s.iter().map(|s| s.len()).max().unwrap();
    for i in 0..n {
        while s[i].len() < l {
            s[i].push('*');
        }
    }

    for j in 0..l {
        let mut alpha = false;
        let mut ans = Vec::new();
        for i in 0..n {
            if s[i][j] == '*' {
                if alpha {
                    ans.push('*');
                }
            } else {
                ans.push(s[i][j]);
                alpha = true;
            }
        }
        ans.reverse();
        let ans = ans.iter().collect::<String>();
        println!("{}", ans);
    }
}
