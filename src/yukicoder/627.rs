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
    let t: usize = read();
    let mut xs = vec![0; t];
    for i in 0..t {
        xs[i] = read();
    }
    let ans = xs
        .iter()
        .scan(0, |prev, &x| {
            let res = *prev - 1 == x || *prev + 1 == x;
            *prev = x;
            Some(res)
        })
        .all(|y| y);
    println!("{}", if ans { 'T' } else { 'F' });
}
