use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let k: u64 = rd.get();
    let ab: Vec<(u64, u64)> = (0..n)
        .map(|_| {
            let a: u64 = rd.get();
            let b: u64 = rd.get();
            (a, b)
        })
        .collect();

    let mut ab = ab;
    ab.sort();
    use std::collections::VecDeque;
    let mut q: VecDeque<(u64, u64)> = ab.into_iter().collect();
    let mut i = 0;
    let mut k = k;
    while let Some((a, b)) = q.pop_front() {
        if i + k < a {
            println!("{}", i + k);
            return;
        } else {
            k -= a - i;
            k += b;
            i = a;
        }
    }
    let ans = i + k;
    println!("{}", ans);
}
