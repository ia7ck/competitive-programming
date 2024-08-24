use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [i64; n],
    };

    let mut t = 1;
    for mut h in h {
        if t % 3 == 2 {
            // 1 + 3 + 1 + 1 + 3 + ...
            h -= 1;
            t += 1;
        }
        if h >= 1 && t % 3 == 0 {
            // 3 + 1 + 1 + 3 + ...
            h -= 3;
            t += 1;
        }
        if h >= 1 {
            assert_eq!(t % 3, 1);
            // 1 + 1 + 3 + 1 + 1 + 3 + ...
            t += h / 5 * 3;
            t += match h % 5 {
                0 => 0,
                1 => 1,
                2 => 2,
                _ => 3,
            };
        }
    }
    println!("{}", t - 1);
}

// 1 2 3 4 5  6  7  8  9
// 1 2 5 5 7 10 11 12 15
