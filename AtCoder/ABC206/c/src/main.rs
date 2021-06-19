use procon_reader::ProconReader;
use std::collections::HashMap;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let a: Vec<u32> = rd.get_vec(n);

    let mut freq = HashMap::new();
    let mut ans = n * (n - 1) / 2;
    for x in a {
        let e = freq.entry(x).or_insert(0);
        ans -= *e;
        *e += 1;
    }
    println!("{}", ans);
}
