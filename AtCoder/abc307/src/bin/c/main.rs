use proconio::{input, marker::Chars};


fn main() {
    input! {
        h_a: usize,
        w_a: usize,
        a: [Chars; h_a],
        h_b: usize,
        w_b: usize,
        b: [Chars; h_b],
        h_o: usize,
        w_o: usize,
        o: [Chars; h_o],
    };

    let b = {
        let mut bb = vec![vec!['.'; w_b + 40]; h_b + 40];
        for i in 0..h_b {
            for j in 0..w_b {
                bb[i + 20][j + 20] = b[i][j];
            }
        }
        bb
    };


    for i in 0..b.len() {
        for j in 0..b[i].len() {
            if i + h_a < b.len() && j + w_a < b[i].len() {
                let mut c = b.clone();
                for di in 0..h_a {
                    for dj in 0..w_a {
                        if a[di][dj] == '#' {
                            c[i + di][j + dj] = '#';
                        }
                    }
                }
                let mut black = 0;
                for y in 0..c.len() {
                    for x in 0..c[y].len() {
                        if c[y][x] == '#' {
                            black += 1;
                        }
                    }
                }
                for y in 0..c.len() {
                    for x in 0..c[y].len() {
                        if y + h_o < c.len() && x + w_o < c[y].len() {
                            let mut match_ = true;
                            let mut bb = 0;
                            for dy in 0..h_o {
                                for dx in 0..w_o {
                                    if c[y + dy][x + dx] != o[dy][dx] {
                                        match_ = false;
                                        break;
                                    }
                                    if c[y + dy][x + dx] == '#' {
                                        bb += 1;
                                    }
                                }
                            }
                            if match_ && black == bb {
                                println!("Yes");
                                return;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("No");


}
