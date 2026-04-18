use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            a: Chars,
            b: Chars,
        };

        solve(a, b);
    }
}

fn solve(a: Vec<char>, b: Vec<char>) {
    let a1 = f(a);
    let b1 = f(b);

    if a1 == b1 {
        println!("Yes");
    } else {
        println!("No")
    }
}

fn f(a: Vec<char>) -> Vec<char> {
    let mut res = Vec::new();
    for c in a {
        res.push(c);
        while res.len() >= 4 && &res[res.len() - 4..] == &['(', 'x', 'x', ')'] {
            res.pop();
            res.pop();
            res.pop();
            res.pop();
            res.push('x');
            res.push('x');
        }
    }
    res
}
