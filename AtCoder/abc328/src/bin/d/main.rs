use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut stack = Vec::new(); // stack ??
    for ch in s {
        if ch == 'C' {
            if stack.len() >= 2 && stack[(stack.len() - 2)..] == vec!['A', 'B'] {
                stack.pop();
                stack.pop();
            } else {
                stack.push(ch);
            }
        } else {
            stack.push(ch);
        }
    }
    let ans = stack.into_iter().collect::<String>();
    println!("{}", ans);
}
