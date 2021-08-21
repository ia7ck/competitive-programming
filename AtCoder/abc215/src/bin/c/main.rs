use join::Join;
use next_permutation::NextPermutation;
use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let s: String = rd.get();
    let k: usize = rd.get();

    let mut s: Vec<char> = s.chars().collect();
    s.sort();
    for _ in 0..(k - 1) {
        assert!(s.next_permutation());
    }
    println!("{}", s.iter().join(""));
}
