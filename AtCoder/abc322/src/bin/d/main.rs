use proconio::{input, marker::Chars};

fn rotate(p: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    assert_eq!(p.len(), p[0].len());
    let n = p.len();
    let mut q = vec![vec!['?'; n]; n];
    for i in 0..n {
        for j in 0..n {
            q[i][j] = p[j][n - i - 1];
        }
    }
    q
}

fn main() {
    input! {
        p: [Chars; 4],
        q: [Chars; 4],
        r: [Chars; 4],
    };

    let ps = [
        p.clone(),
        rotate(&p),
        rotate(&rotate(&p)),
        rotate(&rotate(&rotate(&p))),
    ];
    let qs = [
        q.clone(),
        rotate(&q),
        rotate(&rotate(&q)),
        rotate(&rotate(&rotate(&q))),
    ];
    let rs = [
        r.clone(),
        rotate(&r),
        rotate(&rotate(&r)),
        rotate(&rotate(&rotate(&r))),
    ];
    for p in &ps {
        for q in &qs {
            for r in &rs {
                for pi in 0..8 {
                    for pj in 0..8 {
                        for qi in 0..8 {
                            for qj in 0..8 {
                                for ri in 0..8 {
                                    for rj in 0..8 {
                                        let mut c = [[0; 12]; 12];
                                        for i in 0..4 {
                                            for j in 0..4 {
                                                if p[i][j] == '#' {
                                                    c[pi + i][pj + j] += 1;
                                                }
                                                if q[i][j] == '#' {
                                                    c[qi + i][qj + j] += 1;
                                                }
                                                if r[i][j] == '#' {
                                                    c[ri + i][rj + j] += 1;
                                                }
                                            }
                                        }
                                        let mut ok = true;
                                        'l: for i in 0..12 {
                                            for j in 0..12 {
                                                if 4 <= i && i < 8 && 4 <= j && j < 8 {
                                                    if c[i][j] != 1 {
                                                        ok = false;
                                                        break 'l;
                                                    }
                                                } else {
                                                    if c[i][j] != 0 {
                                                        ok = false;
                                                        break 'l;
                                                    }
                                                }
                                            }
                                        }
                                        if ok {
                                            println!("Yes");
                                            return;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("No");
}
