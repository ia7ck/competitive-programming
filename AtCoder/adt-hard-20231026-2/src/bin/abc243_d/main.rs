use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        x: u64,
        s: Chars,
    };

    let mut x = x;
    let mut down = Vec::new();
    for ch in s {
        match ch {
            'L' | 'R' => {
                down.push(ch);
            }
            'U' => {
                if let Some(_) = down.pop() {
                    //
                } else {
                    x = x / 2;
                }
            }
            _ => unreachable!(),
        }
    }
    for ch in down {
        if ch == 'L' {
            x = x * 2;
        } else {
            x = x * 2 + 1;
        }
    }
    println!("{}", x);
}
