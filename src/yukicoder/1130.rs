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
    let h: usize = read();
    let w: usize = read();
    let mut a = vec![0; h * w];
    for i in 0..h * w {
        a[i] = read();
    }
    a.sort();
    for i in 0..h {
        let b = &a[i * w..(i + 1) * w];
        println!(
            "{}",
            b.iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}
