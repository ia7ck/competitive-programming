use binary_search::BinarySearch;
use procon_reader::ProconReader;
use std::collections::HashMap;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let points: Vec<(u32, u32)> = (0..n)
        .map(|_| {
            let x: u32 = rd.get();
            let y: u32 = rd.get();
            (x, y)
        })
        .collect();
    let mut xs: Vec<u32> = points.iter().map(|&(x, _)| x).collect();
    xs.sort();
    xs.dedup();

    let mut ys = vec![vec![]; n];
    for &(x, y) in &points {
        let i = xs.lower_bound(&x);
        ys[i].push(y);
    }
    for x in 0..n {
        ys[x].sort();
    }

    let mut ans = 0;
    let mut freq: HashMap<(u32, u32), u64> = HashMap::new();
    for x in 0..n {
        let mut y_pairs = Vec::new();
        for i in 0..ys[x].len() {
            for j in (i + 1)..ys[x].len() {
                y_pairs.push((ys[x][i], ys[x][j]));
            }
        }
        for yy in &y_pairs {
            if let Some(f) = freq.get(yy) {
                ans += f;
            }
        }
        for yy in y_pairs {
            freq.entry(yy).and_modify(|f| *f += 1).or_insert(1);
        }
    }
    println!("{}", ans);
}
