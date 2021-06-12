use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let a: Vec<u64> = rd.get_vec(n);

    let mut a = a;
    a.sort();
    let mid = a[n / 2];
    let x = mid as f64 / 2.0;
    let mut ans = 0.0;
    for y in a {
        let y = y as f64;
        ans += x + y - y.min(x * 2.0);
    }
    println!("{}", ans / n as f64);
}
