use std::collections::HashSet;

use proconio::{input, marker::Bytes};

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Bytes; h],
    };

    let mut row = vec![vec![0; 26]; h];
    for i in 0..h {
        for j in 0..w {
            row[i][(a[i][j] - b'a') as usize] += 1;
        }
    }
    let mut column = vec![vec![0; 26]; w];
    for j in 0..w {
        for i in 0..h {
            column[j][(a[i][j] - b'a') as usize] += 1;
        }
    }

    #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
    enum T {
        Row(usize),
        Column(usize),
    }

    let mut row_x = vec![0; h];
    let mut column_x = vec![0; w];
    let mut erase = Vec::new();
    let mut seen = HashSet::new();
    for i in 0..h {
        let x = row[i].iter().filter(|&&c| c >= 1).count();
        row_x[i] = x;
        if x == 1 {
            erase.push(T::Row(i));
        }
    }
    for j in 0..w {
        let x = column[j].iter().filter(|&&c| c >= 1).count();
        column_x[j] = x;
        if x == 1 {
            erase.push(T::Column(j));
        }
    }

    let mut a = a;
    while erase.len() >= 1 {
        let mut mark = Vec::new();
        let mut erase_next = Vec::new();
        for t in erase {
            match t {
                T::Row(i) => {
                    let cookies = (0..w).filter(|&j| a[i][j] != b'.').count();
                    if cookies < 2 {
                        continue;
                    }
                    for j in 0..w {
                        if a[i][j] == b'.' {
                            continue;
                        }
                        let c = (a[i][j] - b'a') as usize;
                        mark.push((i, j));
                        assert!(column[j][c] >= 1);
                        column[j][c] -= 1;
                        if column[j][c] == 0 {
                            assert!(column_x[j] >= 1);
                            column_x[j] -= 1;
                            if column_x[j] == 1 {
                                let next = T::Column(j);
                                if seen.contains(&next) == false {
                                    seen.insert(next);
                                    erase_next.push(next);
                                    continue;
                                }
                            }
                        }
                    }
                }
                T::Column(j) => {
                    let cookies = (0..h).filter(|&i| a[i][j] != b'.').count();
                    if cookies < 2 {
                        continue;
                    }
                    for i in 0..h {
                        if a[i][j] == b'.' {
                            continue;
                        }
                        let c = (a[i][j] - b'a') as usize;
                        mark.push((i, j));
                        assert!(row[i][c] >= 1);
                        row[i][c] -= 1;
                        if row[i][c] == 0 {
                            assert!(row_x[i] >= 1);
                            row_x[i] -= 1;
                            if row_x[i] == 1 {
                                let next = T::Row(i);
                                if seen.contains(&next) == false {
                                    seen.insert(next);
                                    erase_next.push(next);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }
        for (i, j) in mark {
            a[i][j] = b'.';
        }
        erase = erase_next;
    }

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if a[i][j] != b'.' {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
