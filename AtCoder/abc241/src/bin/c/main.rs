use scanner_proc_macro::insert_scanner;

#[insert_scanner]
fn main() {
    let n = scan!(usize);
    let s: Vec<Vec<char>> = (0..n)
        .map(|_| {
            let s = scan!(String);
            s.chars().collect()
        })
        .collect();

    for i in 0..n {
        for j in 0..n {
            let mut ok = false;
            if j + 6 <= n {
                let black_yoko = s[i][j..(j + 6)].iter().filter(|&&ch| ch == '#').count();
                ok |= black_yoko >= 4;
            }
            if i + 6 <= n {
                let mut black_tate = 0;
                for k in i..(i + 6) {
                    if s[k][j] == '#' {
                        black_tate += 1;
                    }
                }
                ok |= black_tate >= 4;
            }
            if i + 6 <= n && j + 6 <= n {
                let mut black_naname = 0;
                for k in 0..6 {
                    if s[i + k][j + k] == '#' {
                        black_naname += 1;
                    }
                }
                ok |= black_naname >= 4;
            }
            if i + 6 <= n && j >= 5 {
                let mut black_naname = 0;
                for k in 0..6 {
                    if s[i + k][j - k] == '#' {
                        black_naname += 1;
                    }
                }
                ok |= black_naname >= 4;
            }
            if ok {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
