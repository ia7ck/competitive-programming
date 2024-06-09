use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let ans = carpet(n);
    for row in ans {
        println!("{}", row.iter().collect::<String>());
    }
}

fn carpet(k: usize) -> Vec<Vec<char>> {
    if k == 0 {
        return vec![vec!['#']];
    }
    let w = 3_usize.pow(k as u32);
    let small = carpet(k - 1);
    let mut grid = vec![vec!['.'; w]; w];
    for i in 0..3 {
        for j in 0..3 {
            if (i, j) == (1, 1) {
                continue;
            }
            for di in 0..(w / 3) {
                for dj in 0..(w / 3) {
                    grid[i * (w / 3) + di][j * (w / 3) + dj] = small[di][dj];
                }
            }
        }
    }
    grid
}
