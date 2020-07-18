fn gcd(a: i64, b: i64) -> i64 {
    if a % b == 0 {
        b
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).ok();
    let ab: Vec<i64> = line
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let (a, b) = (ab[0], ab[1]);
    let g = gcd(a, b);
    let sqrt = (g as f64).sqrt();
    if sqrt.ceil() == sqrt.floor() {
        println!("Odd");
    } else {
        println!("Even");
    }
}
