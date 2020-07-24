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
    let a: Vec<i32> = (0..4).map(|_| read()).collect();
    if a[0] < a[1] && a[2] > a[3] {
        println!("YES");
    } else {
        println!("NO");
    }
}
