use grid_search::around;
use join::Join;
use proconio::input;
use std::iter;

fn main() {
    input! {
        n: usize,
    };

    let mut a = vec![vec![0; n]; n];
    let mut v = 1;
    let ks = if n % 2 == 1 {
        if n % 4 == 1 {
            (0..=(n - 1) / 2)
                .step_by(2)
                .chain((1..=(n - 1) / 2).step_by(2))
                .collect::<Vec<_>>()
        } else {
            (1..=(n - 1) / 2)
                .step_by(2)
                .chain((0..=(n - 1) / 2 - 1).step_by(2))
                .collect::<Vec<_>>()
        }
    } else {
        (0..=n / 2 - 1)
            .step_by(2)
            .chain((1..=n / 2 - 1).step_by(2))
            .collect::<Vec<_>>()
    };
    for k in ks {
        let w = n - k * 2;
        let ij = if w == 1 {
            vec![(k, k)]
        } else {
            iter::repeat(k)
                .zip(k..(k + w - 1))
                .chain((k..(k + w - 1)).zip(iter::repeat(k + w - 1)))
                .chain(iter::repeat(k + w - 1).zip(((k + 1)..=(k + w - 1)).rev()))
                .chain((((k + 1)..=(k + w - 1)).rev()).zip(iter::repeat(k)))
                .collect::<Vec<_>>()
        };
        for (i, j) in ij {
            a[i][j] = v;
            v += 1;
        }
    }
    assert_eq!(v, n * n + 1);

    let dir = [
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
    ];
    for i in 0..n {
        for j in 0..n {
            let mut large = 0;
            let mut small = 0;
            for (ni, nj) in around(i, j).y_range(0..n).x_range(0..n).directions(&dir) {
                if a[ni][nj] > a[i][j] {
                    large += 1;
                } else {
                    small += 1;
                }
            }
            assert_ne!(large, small, "{} {}", i, j);
        }
    }

    for row in a {
        println!("{}", row.iter().join(" "));
    }
}
