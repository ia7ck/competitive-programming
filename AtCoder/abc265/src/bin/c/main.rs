use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        g: [Chars; h],
    };

    let mut seen = vec![vec![false; w]; h];
    let mut cy = 0;
    let mut cx = 0;
    let mut inf = false;
    loop {
        if seen[cy][cx] {
            inf = true;
            break;
        }
        seen[cy][cx] = true;
        match g[cy][cx] {
            'U' => {
                if cy == 0 {
                    break;
                }
                cy -= 1;
            }
            'D' => {
                if cy + 1 == h {
                    break;
                }
                cy += 1;
            }
            'L' => {
                if cx == 0 {
                    break;
                }
                cx -= 1;
            }
            'R' => {
                if cx + 1 == w {
                    break;
                }
                cx += 1;
            }
            _ => unreachable!(),
        }
    }
    if inf {
        println!("-1");
    } else {
        println!("{} {}", cy + 1, cx + 1);
    }
}
