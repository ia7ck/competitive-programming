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
    let mut x: Vec<f64> = vec![0.0; n];
    for i in 0..n {
        x[i] = read();
    }
    let ave = x.iter().sum::<f64>() / n as f64;
    for x in x {
        let d = 50.0 - (ave - x) / 2.0;
        println!("{}", d.floor());
    }
}
