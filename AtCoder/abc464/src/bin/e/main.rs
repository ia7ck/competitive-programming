use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
        queries: [(Usize1, Usize1, char); q],
    };

    let mut a = vec![vec!['?'; w]; h];
    for &(r, c, x) in queries.iter().rev() {
        let mut fill = Vec::new();
        for i in (0..=r).rev() {
            if a[i][c] != '?' {
                break;
            }
            for j in (0..=c).rev() {
                if a[i][j] != '?' {
                    break;
                }
                fill.push((i, j));
            }
        }
        for j in (0..=c).rev() {
            if a[r][j] != '?' {
                break;
            }
            for i in (0..=r).rev() {
                if a[i][j] != '?' {
                    break;
                }
                fill.push((i, j));
            }
        }
        for (i, j) in fill {
            a[i][j] = x;
        }
    }

    for i in 0..h {
        for j in 0..w {
            if a[i][j] == '?' {
                a[i][j] = 'A';
            }
        }
    }
    for row in a {
        println!("{}", row.iter().collect::<String>());
    }
}
