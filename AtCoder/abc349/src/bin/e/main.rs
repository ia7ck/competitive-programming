use proconio::input;

fn main() {
    input! {
        a: [[i64; 3]; 3],
    };
    let a = [
        [a[0][0], a[0][1], a[0][2]],
        [a[1][0], a[1][1], a[1][2]],
        [a[2][0], a[2][1], a[2][2]],
    ];
    let c = [[None; 3]; 3];
    let win = solve(true, a, c);
    if win {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Red,
    Blue,
}

fn solve(first: bool, a: [[i64; 3]; 3], c: [[Option<Color>; 3]; 3]) -> bool {
    if c.iter().flatten().all(|&x| x.is_some()) {
        assert!(!first);
        let mut red = 0;
        let mut blue = 0;
        for i in 0..3 {
            for j in 0..3 {
                match c[i][j] {
                    Some(Color::Red) => red += a[i][j],
                    Some(Color::Blue) => blue += a[i][j],
                    None => unreachable!(),
                }
            }
        }
        assert_ne!(red, blue);
        return red < blue;
    }
    if let Some(color) = bingo(&c) {
        if first {
            return color == Color::Red;
        } else {
            return color == Color::Blue;
        }
    }
    let mut c = c;
    for i in 0..3 {
        for j in 0..3 {
            if c[i][j].is_none() {
                c[i][j] = Some(if first { Color::Red } else { Color::Blue });
                if !solve(!first, a, c) {
                    return true;
                }
                c[i][j] = None;
            }
        }
    }
    false
}

fn bingo(c: &[[Option<Color>; 3]; 3]) -> Option<Color> {
    for i in 0..3 {
        if c[i][0].is_some() && c[i][0] == c[i][1] && c[i][1] == c[i][2] {
            return c[i][0];
        }
    }
    for j in 0..3 {
        if c[0][j].is_some() && c[0][j] == c[1][j] && c[1][j] == c[2][j] {
            return c[0][j];
        }
    }
    if c[0][0].is_some() && c[0][0] == c[1][1] && c[1][1] == c[2][2] {
        return c[0][0];
    }
    if c[0][2].is_some() && c[0][2] == c[1][1] && c[1][1] == c[2][0] {
        return c[0][2];
    }
    None
}
