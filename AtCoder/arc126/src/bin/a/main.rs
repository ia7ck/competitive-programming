use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let t: usize = rd.get();
    for _ in 0..t {
        let a: u64 = rd.get();
        let b: u64 = rd.get();
        let c: u64 = rd.get();

        let b = b / 2;
        if b > c {
            let mut ans = c;
            let b = b - c;
            ans += (a / 2).min(b);
            if a >= b * 2 {
                ans += (a - b * 2) / 5;
            }
            println!("{}", ans);
        } else if b == c {
            println!("{}", b + a / 5);
        } else {
            let mut ans = b;
            let c = c - b;
            ans += a.min(c / 2);
            if a >= c / 2 {
                let (a, c) = (a - c / 2, c % 2);
                if c == 0 {
                    ans += a / 5;
                } else {
                    if a >= 3 {
                        ans += 1;
                        ans += (a - 3) / 5;
                    }
                }
            }
            println!("{}", ans);
        }
    }
    // 2 + 2 + 2 + 2 + 2
    // 2 + 2 + 2 + 4
    // 2 + 2 + 3 + 3
    // 2 + 4 + 4
    // 3 + 3 + 4
    //
}
