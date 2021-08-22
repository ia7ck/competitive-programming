use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let t: usize = rd.get();
    for _ in 0..t {
        let a: u64 = rd.get();
        let b: u64 = rd.get();
        let c: u64 = rd.get();
        solve(a, b, c);
    }
}

fn solve(a: u64, b: u64, c: u64) {
    let d2 = a.max(b) - a.min(b);
    if a <= d2 * 2 && b <= d2 * 2 && c <= d2 * 2 {
        if c <= d2 {
            println!("{}", c + d2);
        } else {
            println!("{}", c - d2);
        }
    } else {
        println!("-1");
    }
}

// a > b
// a - b = d / 2

// 2 5 4
// 1
// 2 5 1
// 4
// c <= d / 2 --> c + d / 2
// c >  d / 2 --> c - d / 2
