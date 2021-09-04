use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let l: u32 = rd.get();
    let q: usize = rd.get();
    use std::collections::BTreeSet;
    let mut cuts = BTreeSet::new();
    for _ in 0..q {
        let c: u8 = rd.get();
        let x: u32 = rd.get();
        if c == 1 {
            cuts.insert(x);
        } else {
            let left = cuts.range(..x).last().copied().unwrap_or(0);
            let right = cuts.range((x + 1)..).next().copied().unwrap_or(l);
            println!("{}", right - left);
        }
    }
}
