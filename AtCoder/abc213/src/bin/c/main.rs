use binary_search::BinarySearch;
use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let _h: usize = rd.get();
    let _w: usize = rd.get();
    let n: usize = rd.get();
    let ab: Vec<(usize, usize)> = (0..n)
        .map(|_| {
            let a: usize = rd.get();
            let b: usize = rd.get();
            (a, b)
        })
        .collect();
    let c = solve(ab.iter().copied().map(|(a, _)| a).collect());
    let d = solve(ab.iter().copied().map(|(_, b)| b).collect());
    for i in 0..n {
        println!("{} {}", c[i], d[i]);
    }
}

fn solve(a: Vec<usize>) -> Vec<usize> {
    let mut values = a.clone();
    values.sort();
    values.dedup();
    let mut c = Vec::new();
    for i in 0..a.len() {
        let y = values.lower_bound(&a[i]);
        c.push(y + 1);
    }
    c
}
