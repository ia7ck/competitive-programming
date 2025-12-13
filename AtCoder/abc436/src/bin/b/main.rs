use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut a = vec![vec![0; n]; n];
    let mut cells = Vec::new();
    a[0][(n - 1) / 2] = 1;
    cells.push((0, (n - 1) / 2, 1));
    for _ in 0..(n * n - 1) {
        let (r, c, k) = cells.last().copied().unwrap();
        let (nr, nc, nk) = if a[(n + r - 1) % n][(c + 1) % n] == 0 {
            ((n + r - 1) % n, (c + 1) % n, k + 1)
        } else {
            ((r + 1) % n, c, k + 1)
        };
        a[nr][nc] = nk;
        cells.push((nr, nc, nk));
    }

    for row in a {
        println!(
            "{}",
            row.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}
