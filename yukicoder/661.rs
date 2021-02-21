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
    let n: usize = read();
    let mut a = vec![0; n];
    for i in 0..n {
        a[i] = read();
    }
    a.iter().for_each(|x| {
        println!(
            "{}",
            if x % 8 == 0 && x % 10 == 0 {
                "ikisugi".to_string()
            } else if x % 8 == 0 {
                "iki".to_string()
            } else if x % 10 == 0 {
                "sugi".to_string()
            } else {
                (x / 3).to_string()
            }
        )
    });
}
