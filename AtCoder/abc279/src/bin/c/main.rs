use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
        t: [Chars; h],
    };

    let mut s_cols = Vec::new();
    let mut t_cols = Vec::new();
    for j in 0..w {
        let mut s_col = Vec::new();
        let mut t_col = Vec::new();
        for i in 0..h {
            s_col.push(s[i][j]);
            t_col.push(t[i][j]);
        }
        s_cols.push(s_col);
        t_cols.push(t_col);
    }

    s_cols.sort();
    t_cols.sort();
    if s_cols == t_cols {
        println!("Yes");
    } else {
        println!("No");
    }
}
