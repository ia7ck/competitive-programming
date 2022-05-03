use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: usize,
    };
    for _ in 0..t {
        input! {
            s: Chars,
            t: Chars,
        };
        solve(s, t);
    }
}

fn solve(s: Vec<char>, t: Vec<char>) {
    if t == vec!['a'] {
        println!("1");
        return;
    }

    if t.contains(&'a') {
        println!("-1");
        return;
    }

    let ans = 2_u64.pow(s.len() as u32);
    println!("{}", ans);
}
