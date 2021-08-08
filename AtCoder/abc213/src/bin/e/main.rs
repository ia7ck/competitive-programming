use grid_search::around;
use procon_reader::ProconReader;

#[rustfmt::skip]
const DIRS: [(isize, isize); 20] = [
    (-2, -1), (-2, 0), (-2, 1),
    (-1, -2), (-1, -1), (-1, 0), (-1, 1), (-1, 2),
    (0, -2), (0, -1), (0, 1), (0, 2),
    (1, -2), (1, -1), (1, 0), (1, 1), (1, 2),
    (2, -1), (2, 0), (2, 1),
];

const UDLR: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let h: usize = rd.get();
    let w: usize = rd.get();
    let s: Vec<Vec<char>> = (0..h).map(|_| rd.get_chars()).collect();

    let inf = std::u64::MAX / 2;
    let mut d = vec![inf; h * w];
    d[0] = 0;
    use std::collections::VecDeque;
    let mut que = VecDeque::new();
    que.push_back(0);
    while let Some(u) = que.pop_front() {
        let i = u / w;
        let j = u % w;
        for (y, x) in around(i, j).y_range(0..h).x_range(0..w).directions(&UDLR) {
            let v = y * w + x;
            if s[y][x] == '.' && d[u] < d[v] {
                d[v] = d[u];
                que.push_front(v);
            }
        }
        for (y, x) in around(i, j).y_range(0..h).x_range(0..w).directions(&DIRS) {
            let v = y * w + x;
            if d[u] + 1 < d[v] {
                d[v] = d[u] + 1;
                que.push_back(v);
            }
        }
    }
    println!("{}", d[h * w - 1]);
}
