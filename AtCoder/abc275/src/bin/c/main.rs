use proconio::{input, marker::Chars};

fn dist(i: usize, j: usize, i2: usize, j2: usize) -> usize {
    (i.max(i2) - i.min(i2)).pow(2) + (j.max(j2) - j.min(j2)).pow(2)
}

fn main() {
    let n = 9;
    input! {
        s: [Chars; n],
    };

    let mut squares = Vec::new();
    for i in 0..n {
        for j in 0..n {
            for i_a in 0..n {
                for j_a in 0..n {
                    if (i, j) == (i_a, j_a) {
                        continue;
                    }
                    let d = dist(i, j, i_a, j_a);
                    for i_b in 0..n {
                        for j_b in 0..n {
                            if (i, j) == (i_b, j_b) {
                                continue;
                            }
                            if (i_a, j_a) == (i_b, j_b) {
                                continue;
                            }
                            if dist(i, j, i_b, j_b) != d {
                                continue;
                            }
                            for i_c in 0..n {
                                for j_c in 0..n {
                                    if (i, j) == (i_c, j_c) {
                                        continue;
                                    }
                                    if (i_a, j_a) == (i_c, j_c) {
                                        continue;
                                    }
                                    if (i_b, j_b) == (i_c, j_c) {
                                        continue;
                                    }
                                    if dist(i_a, j_a, i_c, j_c) != d {
                                        continue;
                                    }
                                    if dist(i_b, j_b, i_c, j_c) != d {
                                        continue;
                                    }
                                    if dist(i, j, i_c, j_c) != dist(i_a, j_a, i_b, j_b) {
                                        continue;
                                    }
                                    if s[i][j] == '#' && s[i_a][j_a] == '#' && s[i_b][j_b] == '#' && s[i_c][j_c] == '#' {
                                        squares.push([(i, j), (i_a, j_a), (i_b, j_b), (i_c, j_c)]);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    for square in squares.iter_mut() {
        square.sort();
    }
    squares.sort();
    squares.dedup();
    println!("{}", squares.len());
}
