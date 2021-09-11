use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let mut s: Vec<Vec<char>> = (0..n).map(|_| rd.get_chars()).collect();
    let t: Vec<Vec<char>> = (0..n).map(|_| rd.get_chars()).collect();

    let mut sc = 0;
    let mut tc = 0;
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == '#' {
                sc += 1;
            }
            if t[i][j] == '#' {
                tc += 1;
            }
        }
    }
    if sc != tc {
        println!("No");
        return;
    }

    let mut u = vec![vec!['.'; n * 3]; n * 3];
    for i in 0..n {
        for j in 0..n {
            u[i + n][j + n] = t[i][j];
        }
    }
    for _ in 0..4 {
        let mut points = Vec::new();
        for i in 0..n {
            for j in 0..n {
                if s[i][j] == '#' {
                    points.push((i, j));
                }
            }
        }
        for i in 0..=(n * 2) {
            for j in 0..=(n * 2) {
                let mut ok = true;
                for &(di, dj) in &points {
                    if u[i + di][j + dj] == '.' {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    println!("Yes");
                    return;
                }
            }
        }
        s = rotate(s);
    }
    println!("No");
}

fn rotate(s: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let n = s.len();
    let mut t = vec![vec!['.'; n]; n];
    for i in 0..n {
        for j in 0..n {
            t[i][j] = s[n - j - 1][i];
        }
    }
    t
}
