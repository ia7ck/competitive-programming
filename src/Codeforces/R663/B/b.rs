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
    for _ in 0..t {
        let n: usize = read();
        let m: usize = read();
        let a = (0..n)
            .map(|_| {
                let row: String = read();
                row.chars().collect::<Vec<_>>()
            })
            .collect::<Vec<Vec<_>>>();
        let mut ans = a[n - 1].iter().filter(|&&c| c == 'D').count();
        for i in 0..n {
            if a[i][m - 1] == 'R' {
                ans += 1;
            }
        }
        println!("{}", ans);
    }
}
