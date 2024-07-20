use memoise::memoise_map;
use proconio::input;

fn main() {
    input! {
        n: u64,
    };

    if let Some(ans) = solve(n) {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

#[memoise_map(n)]
fn solve(n: u64) -> Option<String> {
    let s = n.to_string();
    if !s.contains('0') && s.chars().zip(s.chars().rev()).all(|(c1, c2)| c1 == c2) {
        return Some(s);
    }
    for a in 2..=n {
        if a * a > n {
            break;
        }
        if n % a == 0 {
            let s = a.to_string();
            if s.contains('0') {
                continue;
            }
            let b = s.chars().rev().collect::<String>().parse::<u64>().unwrap();
            if (n / a) % b == 0 {
                let new_n = n / a / b;
                if let Some(res) = solve(new_n) {
                    return Some(format!("{a}*{res}*{b}"));
                }
            }
        }
    }
    None
}
