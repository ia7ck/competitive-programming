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
    let n = 5 * 1000000 + 1;
    let mut is_prime = vec![true; n];
    is_prime[1] = false;
    for i in 2..n {
        if i * i >= n {
            break;
        }
        if is_prime[i] {
            let mut j = i * 2;
            while j < n {
                is_prime[j] = false;
                j += i;
            }
        }
    }

    let t: usize = read();
    for _ in 0..t {
        let a: i64 = read();
        let p: i64 = read();
        let ans = if is_prime[p as usize] {
            if a % p == 0 {
                0
            } else {
                1
            }
        } else {
            -1
        };
        println!("{}", ans);
    }
}
