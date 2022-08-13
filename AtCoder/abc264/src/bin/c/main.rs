use proconio::input;

fn main() {
    input! {
        h1: usize,
        w1: usize,
        a: [[u32; w1]; h1],
        h2: usize,
        w2: usize,
        b: [[u32; w2]; h2],
    };

    for f in 0..(1_usize << h1) {
        for g in 0..(1_usize << w1) {
            if f.count_ones() as usize != h2 {
                continue;
            }
            if g.count_ones() as usize != w2 {
                continue;
            }
            let mut c = Vec::new();
            for i in 0..h1 {
                if f >> i & 1 == 1 {
                    let mut row = Vec::new();
                    for j in 0..w1 {
                        if g >> j & 1 == 1 {
                            row.push(a[i][j]);
                        }
                    }
                    c.push(row);
                }
            }
            if b == c {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
