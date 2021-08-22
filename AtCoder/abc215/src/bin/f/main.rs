use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let mut points: Vec<(i64, i64)> = (0..n)
        .map(|_| {
            let x: i64 = rd.get();
            let y: i64 = rd.get();
            (x, y)
        })
        .collect();

    points.sort();
    let f = |d: i64| -> bool {
        let mut min_y = std::i64::MAX / 2;
        let mut max_y = std::i64::MIN / 2;
        let mut j = 0;
        for i in 0..n {
            let (x, y) = points[i];
            while j <= i && x - points[j].0 >= d {
                let (_, yy) = points[j];
                min_y = min_y.min(yy);
                max_y = max_y.max(yy);
                j += 1;
            }
            if max_y - y >= d || y - min_y >= d {
                return true;
            }
        }
        false
    };

    let mut ok = 0;
    let mut ng = 2_000_000_000 + 1;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        if f(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
}
