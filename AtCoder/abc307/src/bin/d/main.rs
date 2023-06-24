use proconio::{input, marker::Chars};


fn main() {
    input! {
        _n: usize,
        s: Chars,
    };

    let mut stack = Vec::new();
    let mut open = 0;
    for ch in s {
        if ch == ')' && open >= 1 {
            while stack.last() != Some(&'(') {
                stack.pop();
            }
            stack.pop(); // (
            open -= 1;
        } else {
            stack.push(ch);
            if ch == '(' {
                open += 1;
            }
        }
    }

    let mut ans = String::new();
    for ch in stack {
        ans.push(ch);
    }
    println!("{}", ans);
}
