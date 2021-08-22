use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let t: usize = rd.get();
    for _ in 0..t {
        let k: usize = rd.get();
        solve(k);
    }
}

fn solve(k: usize) {
    fn dislike(x: usize) -> bool {
        x % 3 == 0 || x.to_string().ends_with("3")
    }
    let ans = (1..).filter(|&x| !dislike(x)).nth(k - 1).unwrap();
    println!("{}", ans);
}
