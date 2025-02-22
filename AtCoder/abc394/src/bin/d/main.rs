use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    // `(`, `[`, `<`
    let mut open = Vec::new();
    for c in s {
        match c {
            '(' | '[' | '<' => open.push(c),
            ')' => {
                if let Some('(') = open.pop() {
                    // ok
                } else {
                    println!("No");
                    return;
                }
            }
            ']' => {
                if let Some('[') = open.pop() {
                    // ok
                } else {
                    println!("No");
                    return;
                }
            }
            '>' => {
                if let Some('<') = open.pop() {
                    // ok
                } else {
                    println!("No");
                    return;
                }
            }
            _ => unreachable!(),
        }
    }

    if open.is_empty() {
        println!("Yes");
    } else {
        println!("No");
    }
}
