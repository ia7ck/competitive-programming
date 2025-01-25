use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };

    let i_min = (0..h).find(|&i| s[i].contains(&'#')).unwrap();
    let i_max = (0..h).rfind(|&i| s[i].contains(&'#')).unwrap();
    let j_min = (0..w).find(|&j| (0..h).any(|i| s[i][j] == '#')).unwrap();
    let j_max = (0..w).rfind(|&j| (0..h).any(|i| s[i][j] == '#')).unwrap();

    for i in i_min..=i_max {
        for j in j_min..=j_max {
            if s[i][j] == '#' || s[i][j] == '?' {
                // ok
            } else {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}
