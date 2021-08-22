use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let mut m: usize = rd.get();
    let a: Vec<usize> = rd.get_vec(n);

    let k = 2_000_00;
    let mut freq = vec![0; k + 1];
    for &x in &a {
        freq[x] += 1;
    }
    let empty = freq.iter().filter(|&&f| f == 0).count();
    let mut e = 0;
    for x in 0..=k {
        while m >= 1 && freq[x] >= 2 && e < empty {
            m -= 1;
            freq[x] -= 1;
            e += 1;
        }
    }
    let filled = freq.iter().filter(|&&f| f >= 1).count();
    println!("{}", filled + e);
}
