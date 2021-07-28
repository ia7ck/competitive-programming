use grid_search::around;
use procon_reader::ProconReader;
use rustc_hash::FxHashSet;

const DIR: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

fn dfs(
    k: usize,
    n: usize,
    s: &mut Vec<Vec<char>>,
    patterns: &mut FxHashSet<Vec<Vec<char>>>,
    ans: &mut usize,
) {
    if patterns.contains(s) {
        return;
    }
    patterns.insert(s.clone());
    if k == 0 {
        *ans += 1;
        return;
    }
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == '.' {
                let ok = around(i, j)
                    .directions(&DIR)
                    .y_range(0..n)
                    .x_range(0..n)
                    .any(|(ni, nj)| s[ni][nj] == '@');
                if ok {
                    s[i][j] = '@';
                    dfs(k - 1, n, s, patterns, ans);
                    s[i][j] = '.';
                }
            }
        }
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let k: usize = rd.get();
    let mut s: Vec<Vec<char>> = (0..n).map(|_| rd.get_chars()).collect();

    let mut patterns = FxHashSet::default();
    let mut ans = 0;
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == '.' {
                s[i][j] = '@';
                dfs(k - 1, n, &mut s, &mut patterns, &mut ans);
                s[i][j] = '.';
            }
        }
    }
    println!("{}", ans);
}
