use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let a = vec!["ABC", "ARC", "AGC", "AHC"];
    use std::collections::HashSet;
    let mut a: HashSet<String> = a.into_iter().map(|s| s.to_string()).collect();
    for _ in 0..3 {
        let s: String = rd.get();
        a.remove(&s);
    }
    let a: Vec<String> = a.into_iter().collect();
    assert_eq!(a.len(), 1);
    println!("{}", a[0]);
}
