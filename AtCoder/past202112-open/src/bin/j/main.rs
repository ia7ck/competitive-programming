use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    };

    // 1--2         4--1         3--4         2--3
    // |  |  -2A->  |  |  -2A->  |  |  -2A->  |  |
    // 4--3         3--2         2--1         1--4

    // 1--2         4--3
    // |  |  -3A->  |  |
    // 4--3         1--2

    // 1--2         2--1
    // |  |  -3B->  |  |
    // 4--3         3--4

    let mut a = vec![vec![0; n]; n];
    let mut cw = true;
    let mut rot = 0;
    for _ in 0..q {
        input! {
            op: u8,
        };
        if op == 1 {
            input! {
                r: usize,
                c: usize,
            };
            let (r, c) = (r - 1, c - 1);
            let (r, c) = match (cw, rot) {
                (true, 0) => (r, c),
                (true, 1) => (n - c - 1, r),
                (true, 2) => (n - r - 1, n - c - 1),
                (true, 3) => (c, n - r - 1),
                (false, 0) => (c, r),
                (false, 1) => (r, n - c - 1),
                (false, 2) => (n - c - 1, n - r - 1),
                (false, 3) => (n - r - 1, c),
                _ => unreachable!(),
            };
            a[r][c] ^= 1;
        } else if op == 2 {
            input! {
                dir: char,
            };
            if dir == 'A' {
                rot = (rot + 1) % 4;
            } else {
                rot = (4 + rot - 1) % 4;
            }
        } else {
            input! {
                dir: char,
            };
            if dir == 'A' {
                if rot % 2 == 0 {
                    rot = (4 + rot - 1) % 4;
                } else {
                    rot = (rot + 1) % 4;
                }
            } else {
                if rot % 2 == 0 {
                    rot = (rot + 1) % 4;
                } else {
                    rot = (4 + rot - 1) % 4;
                }
            }
            cw = !cw;
        }
    }

    for r in 0..n {
        for c in 0..n {
            let (r, c) = match (cw, rot) {
                (true, 0) => (r, c),
                (true, 1) => (n - c - 1, r),
                (true, 2) => (n - r - 1, n - c - 1),
                (true, 3) => (c, n - r - 1),
                (false, 0) => (c, r),
                (false, 1) => (r, n - c - 1),
                (false, 2) => (n - c - 1, n - r - 1),
                (false, 3) => (n - r - 1, c),
                _ => unreachable!(),
            };
            print!("{}", a[r][c]);
        }
        println!();
    }
}
