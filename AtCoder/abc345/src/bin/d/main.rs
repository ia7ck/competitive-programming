use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        n: usize,
        h: usize,
        w: usize,
        tiles: [(usize, usize); n],
    };

    for bits in 1..(1 << n) {
        let mut sub_tiles = Vec::new();
        for i in 0..n {
            if bits >> i & 1 == 1 {
                sub_tiles.push(tiles[i]);
            }
        }
        let area = sub_tiles.iter().map(|(h, w)| h * w).sum::<usize>();
        if area != h * w {
            continue;
        }
        sub_tiles.sort_by_key(|(h, w)| h * w);
        sub_tiles.reverse();
        if solve(h, w, &sub_tiles, 0, &mut vec![vec![false; w]; h]) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

fn solve(
    h: usize,
    w: usize,
    tiles: &[(usize, usize)],
    index: usize,
    grid: &mut Vec<Vec<bool>>,
) -> bool {
    if index == tiles.len() {
        return true;
    }
    let (a, b) = tiles[index];
    for &(a, b) in &[(a, b), (b, a)] {
        for (i, j) in iproduct!(0..h, 0..w) {
            if i + a > h {
                continue;
            }
            if j + b > w {
                continue;
            }
            let mut put = true;
            for (di, dj) in iproduct!(0..a, 0..b) {
                if grid[i + di][j + dj] {
                    put = false;
                    break;
                }
            }
            if put {
                for (di, dj) in iproduct!(0..a, 0..b) {
                    grid[i + di][j + dj] = true;
                }
                if solve(h, w, tiles, index + 1, grid) {
                    return true;
                }
                for (di, dj) in iproduct!(0..a, 0..b) {
                    grid[i + di][j + dj] = false;
                }
            }
        }
    }
    false
}
