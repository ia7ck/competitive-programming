use proconio::{input, marker::Bytes};

fn main() {
    input! {
        r: usize,
        c: usize,
        b: [Bytes; r],
    };

    let mut ans = b.clone();
    for i in 0..r {
        for j in 0..c {
            if b'1' <= b[i][j] && b[i][j] <= b'9' {
                let k = (b[i][j] - b'0') as usize;
                for y in 0..r {
                    for x in 0..c {
                        if y.max(i) - y.min(i) + x.max(j) - x.min(j) <= k {
                            ans[y][x] = b'.';
                        }
                    }
                }
            }
        }
    }

    for row in ans {
        for b in row {
            print!("{}", b as char);
        }
        println!();
    }
}
