use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let st: Vec<(String, String)> = (0..n)
        .map(|_| {
            let s: String = rd.get();
            let t: String = rd.get();
            (s, t)
        })
        .collect();

    for i in 0..n {
        for j in (i + 1)..n {
            if st[i] == st[j] {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
