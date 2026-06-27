use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    };

    let mut c = c
        .into_iter()
        .skip_while(|row| !row.contains(&'#'))
        .collect::<Vec<_>>();

    c.reverse();
    let mut c = c
        .into_iter()
        .skip_while(|row| !row.contains(&'#'))
        .collect::<Vec<_>>();
    c.reverse();

    let left = (0..w)
        .find(|&j| (0..c.len()).any(|i| c[i][j] == '#'))
        .unwrap();
    let right = (0..w)
        .rfind(|&j| (0..c.len()).any(|i| c[i][j] == '#'))
        .unwrap();
    for row in c {
        for j in left..=right {
            print!("{}", row[j]);
        }
        println!();
    }
}
