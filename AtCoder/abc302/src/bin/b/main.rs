use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
    };

    let dirs = [
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
    ];

    for i in 0..h {
        for j in 0..w {
            for &(di, dj) in &dirs {
                let mut i = i as isize;
                let mut j = j as isize;
                let mut ans = Vec::new();
                let mut s = String::new();
                ans.push((i, j));
                s.push(a[i as usize][j as usize]);
                for _ in 0..4 {
                    i = i + di;
                    j = j + dj;
                    if i < 0 || i >= h as isize || j < 0 || j >= w as isize {
                        break;
                    }
                    ans.push((i, j));
                    s.push(a[i as usize][j as usize]);
                }
                if s == "snuke" {
                    for (i, j) in ans {
                        println!("{} {}", i + 1, j + 1);
                    }
                    return;
                }
            }
        }
    }

    unreachable!();
}
