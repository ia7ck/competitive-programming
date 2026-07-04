use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    let mut front = Vec::new();
    let mut back = VecDeque::new();
    let mut f: bool;
    if s[n - 1] == 'o' {
        front.push(n);
        f = true;
    } else {
        back.push_front(n);
        f = false;
    }
    for i in (0..(n - 1)).rev() {
        if s[i] == 'o' {
            if f {
                back.push_front(i + 1);
                f = false;
            } else {
                front.push(i + 1);
                f = true;
            }
        } else {
            if f {
                front.push(i + 1);
                f = true;
            } else {
                back.push_front(i + 1);
                f = false;
            }
        }
    }

    let mut ans = front;
    ans.extend(back);

    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
