use std::io::Read;

fn read<T: std::str::FromStr>() -> T {
    let token: String = std::io::stdin()
        .bytes()
        .map(|c| c.ok().unwrap() as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().unwrap()
}

fn main() {
    let n: i32 = read();
    let d: i32 = read();
    let mut s: i32 = 0;
    let mut v = Vec::new();
    v.push(0);
    for _ in 2..=n {
        let a: i32 = read();
        s += a;
        let &last = v.last().unwrap();
        if s - last < d {
            v.push(last + d);
        } else {
            v.push(s);
        }
    }
    let ans: Vec<_> = v.iter().map(|x| x.to_string()).collect();
    println!("{}", ans.join(" "));
}
