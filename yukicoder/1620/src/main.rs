use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let s: Vec<char> = rd.get_chars();

    let n = s.len();
    let mo = 998244353;

    let mut p2 = vec![1];
    let mut p11 = vec![1];
    for i in 0..n {
        p2.push(p2[i] * 2 % mo);
        p11.push(p11[i] * 11 % mo);
    }

    let mut ans = 0;
    for i in 0..n {
        let x = s[i] as u64 - '0' as u64;
        ans = (ans + p2[i] * x % mo * p11[n - i - 1] % mo) % mo;
    }
    println!("{}", ans);
}
