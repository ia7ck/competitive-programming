use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let xy: Vec<(i64, i64)> = (0..n)
        .map(|_| {
            let x: i64 = rd.get();
            let y: i64 = rd.get();
            (x, y)
        })
        .collect();

    let mut xi: Vec<(i64, usize)> = xy
        .iter()
        .copied()
        .map(|(x, _)| x)
        .enumerate()
        .map(|(i, x)| (x, i))
        .collect();
    let mut yi: Vec<(i64, usize)> = xy
        .iter()
        .copied()
        .map(|(_, y)| y)
        .enumerate()
        .map(|(i, y)| (y, i))
        .collect();
    xi.sort();
    yi.sort();
    let mut ans = Vec::new();
    for &(i, j) in &[(0, n - 1), (1, n - 1), (0, n - 2)] {
        let (x, k) = xi[i];
        let (xx, kk) = xi[j];
        ans.push(((x - xx).abs(), (k.min(kk), k.max(kk))));
        let (y, k) = yi[i];
        let (yy, kk) = yi[j];
        ans.push(((y - yy).abs(), (k.min(kk), k.max(kk))));
    }
    ans.sort();
    ans.reverse();
    let (_, first_ij) = ans[0];
    let (second_d, second_ij) = ans[1];
    if first_ij != second_ij {
        println!("{}", second_d);
    } else {
        println!("{}", ans[2].0);
    }
}
