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
    let ha: i32 = read();
    let hb: i32 = read();
    let hc: i32 = read();

    let mut v = vec![(ha, 'A'), (hb, 'B'), (hc, 'C')];
    v.sort_by(|x, y| y.0.cmp(&x.0));
    v.iter().map(|(_, c)| c).for_each(|c| println!("{}", c));
}
