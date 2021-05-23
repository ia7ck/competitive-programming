use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let h: usize = rd.get();
    let w: usize = rd.get();
    let s: Vec<Vec<char>> = (0..h).map(|_| rd.get_chars()).collect();

    let mo: u64 = 998244353;
    let mut ans = 1;
    for j in 0..w {
        let mut coords = Vec::new();
        for i in 0..h {
            if j >= i {
                coords.push((i, j - i));
            }
        }
        let has_red = coords.iter().any(|&(i, j)| s[i][j] == 'R');
        let has_blue = coords.iter().any(|&(i, j)| s[i][j] == 'B');
        let mut a = 0;
        if !has_red {
            a += 1;
        }
        if !has_blue {
            a += 1;
        }
        ans = ans * a % mo;
    }
    for i in 1..h {
        let mut coords = Vec::new();
        for j in (0..w).rev() {
            if i + (w - j - 1) < h {
                coords.push((i  + (w-j-1), j));
            }
        }
        let has_red = coords.iter().any(|&(i, j)| s[i][j] == 'R');
        let has_blue = coords.iter().any(|&(i, j)| s[i][j] == 'B');
        let mut a = 0;
        if !has_red {
            a += 1;
        }
        if !has_blue {
            a += 1;
        }
        ans = ans * a % mo;
    }
    println!("{}", ans);
}
