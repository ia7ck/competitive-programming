use join::Join;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    };

    let mut ans = Vec::new();
    for j in 0..w {
        let mut count = 0;
        for i in 0..h {
            if c[i][j] == '#' {
                count += 1;
            }
        }
        ans.push(count);
    }

    println!("{}", ans.iter().join(" "));
}
