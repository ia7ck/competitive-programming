use binary_search::BinarySearch;
use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let q: usize = rd.get();
    let a: Vec<usize> = rd.get_vec(n);
    for _ in 0..q {
        let k: usize = rd.get();
        let mut ok = 0;
        let mut ng = std::usize::MAX / 2;
        while ng - ok > 1 {
            let mid = (ok + ng) / 2;
            let j = a.lower_bound(&mid);
            if mid.saturating_sub(j) <= k {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        println!("{}", ok);
    }
}
