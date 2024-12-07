use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        d: usize,
        s: [Chars; h],
    };

    let mut ans = 0;
    for i1 in 0..h {
        for j1 in 0..w {
            for i2 in 0..h {
                for j2 in 0..w {
                    if (i1, j1) != (i2, j2) && s[i1][j1] == '.' && s[i2][j2] == '.' {
                        let mut count = 0;
                        for i in 0..h {
                            for j in 0..w {
                                if s[i][j] == '.'
                                    && (i.abs_diff(i1) + j.abs_diff(j1) <= d
                                        || i.abs_diff(i2) + j.abs_diff(j2) <= d)
                                {
                                    count += 1;
                                }
                            }
                        }
                        ans = ans.max(count);
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
