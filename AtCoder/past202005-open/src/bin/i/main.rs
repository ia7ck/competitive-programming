use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        q: usize,
    };

    let mut row = (0..n).collect::<Vec<_>>();
    let mut col = (0..n).collect::<Vec<_>>();
    let mut transpose = false;
    for _ in 0..q {
        input! {
            op: u8,
        };
        if op == 1 {
            input! {
                r1: Usize1,
                r2: Usize1,
            };
            if r1 == r2 {
                continue;
            }
            if transpose {
                col.swap(r1, r2);
            } else {
                row.swap(r1, r2);
            }
        } else if op == 2 {
            input! {
                c1: Usize1,
                c2: Usize1,
            };
            if c1 == c2 {
                continue;
            }
            if transpose {
                row.swap(c1, c2);
            } else {
                col.swap(c1,c2);
            }
        } else if op == 3 {
            transpose = !transpose;
        } else {
            input! {
                r: Usize1,
                c: Usize1,
            };
            let (i, j) = if transpose {
                (row[c], col[r])
            } else {
                (row[r], col[c])
            };
            println!("{}", n * i + j);
        }
    }
}
